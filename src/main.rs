use actix_web::{
    get, 
    post, 
    put, 
    web::{Json, Path}, 
    App,
    HttpResponse,
    HttpServer, 
    Responder,
    
};
use serde::{
    Serialize,
    Deserialize
};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Book {
    id: String,
    title: String,
    subtitle: String,
    authors: Box<[String]>,
    isbn: String
}

impl Book {
    fn new(title: String, subtitle: String, authors: &[String], isbn: String) -> Book {
        let id = Uuid::new_v4().to_string();
        Book { id, title, subtitle, authors: authors.into(), isbn }
    }
}

#[get("/books")]
async fn book_list() -> impl Responder {
    HttpResponse::Ok().body("hello, book_list")
}

#[post("/books")]
async fn new_book(body: Json<Book>) -> impl Responder {
    HttpResponse::Ok().body("hello, new_book ".to_owned() + &body.title)
}

#[put("/books/{id}")]
async fn update_book(path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("hello, update_book {}", id))
}

#[get("/books/{id}")]
async fn get_book_for_id(path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("hello, get_book_for_id {}", id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
            .service(book_list)
            .service(new_book)
            .service(update_book)
            .service(get_book_for_id)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
