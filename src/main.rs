use actix_web::{App, HttpServer, middleware};
mod routes;
use routes::web_ui;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(web_ui)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
