use actix_web::{web, App, HttpResponse, HttpServer, Responder};
 
fn index() -> impl Responder {
    HttpResponse::Ok().body("HELLO WORLD!")
}


fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:15000")
    .unwrap()
    .run()
    .unwrap();
}
