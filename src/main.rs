// /etc/letsencrypt/live/euroban.org/fullchain.pem
use actix_web::{ * };
use rustls::{ *, server::*};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello from async")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}