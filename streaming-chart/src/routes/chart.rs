// use poem::{get, handler, listener::TcpListener, Route, Server};
use poem::endpoint::EmbeddedFileEndpoint;
use poem::Route;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "files"]
pub struct Files;

pub fn route() -> Route {
    Route::new().at(
        "/chart",
        EmbeddedFileEndpoint::<Files>::new("page-echarts.html"),
    )
}
