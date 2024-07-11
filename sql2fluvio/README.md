### Requirements

Create a binary called `sql2fluvio` that takes a SQL file as input and produces to fluvio records.

```bash
sql2fluvio my_sql_file.sql topic-name
```

Sample SQL file:

```sql
select  * from timetest
```

#### Assumptions
* Any select SQL statement should be supported.
* The response will be converter to json with all fields mapped.
* Produce to fluvio topic `topic-name`
* Nice to have - show status as it runs.


## Versions
### v0.2.0

Usage: sql2fluvio <DB_PATH> <SQL_FILE_PATH> <TOPIC_NAME>

Arguments:
  <DB_PATH>        Path to the SQL database file
  <SQL_FILE_PATH>  Path to a generic SQL query
  <TOPIC_NAME>     topic to produce to

Sample usage:
`sql2fluvio  dbfile.sqlite3 test.sql ingest-topic`

### v0.1.0
```
Usage: sql2fluvio <DB_PATH> <TABLE_NAME> <TOPIC_NAME> [SQL_FILE_PATH]

Arguments:
  <DB_PATH>        Path to the SQL database file
  <TABLE_NAME>     table name (used to create col -> json mapping)
  <TOPIC_NAME>     topic to produce to
  [SQL_FILE_PATH]  Path to a generic SQL query  (not supported yet)
```

Sample usage:
```
    cargo run --release -- \
		data/test.sqlite3 \
		samples_view \
		samples
```