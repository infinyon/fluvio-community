use poem::Route;

mod chart;
mod http_stream;

pub fn app_routes() -> Route {
    Route::new()
        .nest_no_strip("/chart", chart::route())
        .nest_no_strip("/http-stream", http_stream::route())
}
