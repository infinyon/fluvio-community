use fluvio::{consumer::ConsumerConfigExtBuilder, Fluvio, Offset};
use futures_util::StreamExt;
use poem::{get, handler, http::StatusCode, web::Path, Body, Response, Route};

pub fn route() -> Route {
    Route::new().at("/http-stream/:name", get(http_stream))
}

#[handler]
async fn http_stream(Path(name): Path<String>) -> poem::Result<Response> {
    tracing::info!("ws connect /ws/{name}");

    let fluvio = Fluvio::connect().await.expect("couldn't connect to fluvio");

    let cfg = ConsumerConfigExtBuilder::default()
        .topic(name)
        .offset_start(Offset::end())
        .build()
        .expect("couldn't config");

    let mut fl_stream = fluvio
        .consumer_with_config(cfg)
        .await
        .expect("Couldn't start fluvio stream");

    let json_stream = fl_stream.map(|rec| -> Result<bytes::Bytes, std::io::Error> {
        let rec = rec.map_err(|err| {
            std::io::Error::new(std::io::ErrorKind::Other, format!("Fluvio error: {}", err))
        })?;
        let rec: String = String::from_utf8_lossy(rec.value()).into_owned();
        let toks: Vec<&str> = rec.split(",").collect();
        let x = toks[0];
        let y = toks[1];
        let jrec = format!("{{ \"x\": {x}, \"y\": {y} }}\n");
        tracing::debug!(jrec, "message");
        let bytes = jrec.as_bytes();

        Ok(bytes::Bytes::copy_from_slice(bytes))
    });

    let body = Body::from_bytes_stream(json_stream);

    let response = Response::builder()
        .status(StatusCode::OK)
        .content_type("application/x-ndjson")
        .header("Cache-Control", "no-cache")
        .body(body);
    Ok(response)
}
