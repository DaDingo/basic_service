use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || 
        {
            App::new()
            .service(
                web::resource("/")
                .route(web::get()
                .to(hello_world))
            )
        } 
    )
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}

async fn hello_world() -> String {
    "hello world".to_string()
}