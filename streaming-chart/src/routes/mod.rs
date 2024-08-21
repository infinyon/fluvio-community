use poem::Route;

mod chart;
mod ws;

pub fn app_routes() -> Route {
    Route::new()
        .nest_no_strip("/chart", chart::route())
        .nest_no_strip("/ws", ws::route())
}
