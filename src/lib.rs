use actix_web::{App, HttpRequest, HttpResponse, HttpServer, dev::Server, web};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check(_req: HttpRequest) ->  HttpResponse {
    HttpResponse::Ok().finish()
}



async fn subscribe(_form: web::Form<FormData>) ->  HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server=HttpServer::new(|| 
        {App::new()
        // .route("/{name}",web::get().to(greet))
        .route("/health_check",web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
    Ok(server)
}


