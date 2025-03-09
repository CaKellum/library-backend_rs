use actix_web::{ App, HttpServer };

mod book;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(book::book_list)
            .service(book::new_book)
            .service(book::update_book)
            .service(book::get_book_for_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
