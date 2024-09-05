use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ferris_says::say;

// Handler function
async fn chantel_krap_message() -> impl Responder {
    let message = "Hello my bras!";
    let width = message.chars().count();

    // Render Ferris's message into a String
    let mut buffer = Vec::new();
    say(message.as_bytes(), width, &mut buffer).unwrap();

    // Return Ferris's ASCII art as a response
    HttpResponse::Ok().body(String::from_utf8(buffer).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(chantel_krap_message)) // Route for Ferris message
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}