use actix_web::{middleware, App, HttpServer};
use env_logger;

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(user::create)
            .service(user::list)
            .service(user::get)
    })
    .bind("0.0.0.0:8989")?
    .run()
    .await
}
