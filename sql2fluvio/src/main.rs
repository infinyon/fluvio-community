use std::sync::Arc;
// use std::time;

use anyhow::{Context, Result};
use clap::Parser;
use fluvio::Compression;
use fluvio::Fluvio;
use fluvio::RecordKey;
use rusqlite::types::ValueRef;
use serde_json::json;
use serde_json::Value as JsonValue;
use serde_json::Number as JsonNumber;
use tokio_rusqlite::Connection;

const FACTOR_1M: usize = 1024 * 1024;
const PRODUCE_BATCH_SIZE_BYTES: usize = 1 * FACTOR_1M;
// const PRODUCE_TIMEOUT_MS: u64 = 5 * 60_000;
// const PRODUCE_LINGER_MS: u64 = 1_000;
// const PRODUCE_BATCH_N_QUEUE: usize = 2;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the SQL database file
    db_path: String,

    /// Path to a generic SQL query
    sql_file_path: String,

    /// topic to produce to
    topic_name: String,

    /// do not create the topic
    #[arg(long, default_value = "false")]
    no_create: bool,
}

/// generic json table to object mapping
fn create_json_object(properties: &[String], values: Vec<JsonValue>) -> JsonValue {
    let mut json_obj = json!({});

    for (prop, value) in properties.iter().zip(values.into_iter()) {
        json_obj[prop] = value;
    }

    json_obj
}

async fn create_topic(fluvio: &Fluvio, topic_name: &str) -> Result<()> {
    use fluvio::metadata::{
        objects::ListFilter,
        topic::{ReplicaSpec, TopicReplicaParam, TopicSpec},
    };
    let admin = fluvio.admin().await;

    let list_filters = vec![ListFilter {
        name: topic_name.to_string(),
    }];
    let listing = admin.list::<TopicSpec, ListFilter>(list_filters).await?;
    if listing.is_empty() {
        println!("Creating topic '{topic_name}'");
        let spec: TopicSpec = ReplicaSpec::Computed(TopicReplicaParam {
            partitions: 1,
            replication_factor: 1,
            ..Default::default()
        })
        .into();
        admin.create(topic_name.to_string(), false, spec).await?;
    } else {
        println!("Topic {topic_name} already exists");
    }
    Ok(())
}

async fn get_fluvio_producer(fluvio: &Fluvio, topic_name: &str) -> Result<fluvio::TopicProducer> {
    let pconfig = fluvio::TopicProducerConfigBuilder::default()
       .batch_size(PRODUCE_BATCH_SIZE_BYTES)
       .compression(Compression::Gzip)
  //      .batch_queue_size(PRODUCE_BATCH_N_QUEUE)
  //      .linger(time::Duration::from_millis(PRODUCE_LINGER_MS))
  //      .timeout(time::Duration::from_millis(PRODUCE_TIMEOUT_MS))
        .build()?;
    fluvio.topic_producer_with_config(topic_name, pconfig).await
}

const BATCH_PRINT: usize = 10_000;

/// produce to fluvio given a generic sql query
async fn sql_produce(
    db_path: &str,
    sql_file_path: &str,
    topic_name: &str,
    fluvio: &Fluvio,
) -> Result<usize> {
    println!("Running query for {db_path}, sql: {sql_file_path}");

    // sqlite requires one connection per prepared statement in multi_threaded access
    // https://www.sqlite.org/threadsafe.html
    let db_conn =
        Connection::open_with_flags(&db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).await?;

    let sql = get_sql(sql_file_path)?;

    let final_rows = db_conn
        .call(move |db_conn| {
            let mut stmt = db_conn.prepare(&sql)?;
            let colnames: Vec<String> = stmt.column_names().iter().map(|n| n.to_string()).collect();
            let default_j_f64 = JsonNumber::from_f64(0.0).unwrap();

            let rows: Vec<JsonValue> = stmt
                .query_map([], |row| {
                    let values: Vec<JsonValue> = (0..colnames.len())
                        .map(|i| {
                            match row.get_ref(i) {
                                Ok(val) => match val {
                                    ValueRef::Null => JsonValue::Null,
                                    ValueRef::Integer(i) => JsonValue::Number(i.into()),
                                    ValueRef::Real(f) => {
                                        let jn = JsonNumber::from_f64(f).unwrap_or_else(|| {
                                            default_j_f64.clone()
                                        });
                                        JsonValue::Number(jn)
                                    },
                                    ValueRef::Text(t) => JsonValue::String(String::from_utf8_lossy(t).to_string()),
                                    ValueRef::Blob(b) => JsonValue::String(format!("BLOB({})", b.len())), // todo BASE 64 or other encoding
                                },
                                _ => JsonValue::String(String::new()),
                            }
                        })
                        .collect();
                    let obj = create_json_object(&colnames, values);
                    Ok(obj)
                })?
                .map(|i| i.unwrap_or_default())
                .collect();
            Ok(rows)
        })
        .await?;

    let n_recs = final_rows.len();
    println!("  producing {n_recs} records to fluvio topic '{topic_name}'");

    let producer = get_fluvio_producer(fluvio, topic_name).await?;
    let mut counter = 0;
    for row in final_rows {
        let string_value = serde_json::to_string(&row)?;
      //  println!("  sending: {string_value}");
        producer.send(RecordKey::NULL, string_value).await?;
        counter += 1;
        // print every 10k
        if counter % BATCH_PRINT == 0 {
            println!("  produced {counter} records");
        }
    }

    // final flush
    producer.flush().await?;
    Ok(n_recs)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let fluvio = Fluvio::connect().await?;
    let fluvio = Arc::new(fluvio);

    if !args.no_create {
        let res = create_topic(&fluvio, &args.topic_name)
            .await
            .with_context(|| format!("error creating topic '{}", &args.topic_name));
        if let Err(err) = res {
            println!("warning: {err:?}");
        }
    }

    sql_produce(
        &args.db_path,
        &args.sql_file_path,
        &args.topic_name,
        &fluvio,
    )
    .await?;

    Ok(())
}

fn get_sql(sql_file: &str) -> Result<String> {
    std::fs::read_to_string(sql_file).with_context(|| format!("reading sql file {}", &sql_file))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TOPIC: &str = "test-sql";

    #[ignore]
    #[tokio::test]
    async fn ingest_values_via_sql() -> Result<()> {
        let db_path = "test/test.db";
        let sql_file = "test.sql";
        let topic_name = TEST_TOPIC;
        let fluvio = Fluvio::connect().await?;

        let res = sql_produce(db_path, sql_file, topic_name, &fluvio).await;
        assert!(res.is_ok(), "{res:?}");
        Ok(())
    }

    #[tokio::test]
    async fn t_create_topic() -> Result<()> {
        let topic_name = TEST_TOPIC;
        let fluvio = Fluvio::connect().await?;

        let res = create_topic(&fluvio, topic_name)
            .await
            .with_context(|| format!("error creating topic '{}", topic_name));
        if let Err(err) = res {
            println!("warning: {err:?}");
        }
        Ok(())
    }
}
