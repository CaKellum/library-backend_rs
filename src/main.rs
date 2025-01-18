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
enum BookStatus {
    CheckedIn = 0,
    Reading = 1,
    Loaned = 2,
} 

#[derive(Deserialize, Serialize)]
struct BookStatusUpdate {
    status: BookStatus
}


#[derive(Deserialize, Serialize)]
struct BookIDResponse {
    id: String
}

#[derive(Deserialize, Serialize)]
struct UnidentifiedBook {
    title: String,
    subtitle: Option<String>,
    authors: Vec<String>,
    isbn: String
}

#[derive(Deserialize, Serialize)]
struct Book {
    id: String,
    title: String,
    subtitle: Option<String>,
    authors: Vec<String>,
    isbn: String
}

impl Book {
    fn new(title: String, subtitle: Option<String>, authors: Vec<String>, isbn: String) -> Book {
        let id = Uuid::new_v4().to_string();
        Book { id, title, subtitle, authors: authors.into(), isbn }
    }

    fn new_with_book(unidentified_book: UnidentifiedBook) -> Book {
        Book::new(unidentified_book.title, unidentified_book.subtitle, unidentified_book.authors, unidentified_book.isbn)
    }
}

///Gets the full list of books in the database
#[get("/books")]
async fn book_list() -> impl Responder {
    // TODO: Make this get data from Database
    let book_list = vec![ Book::new("The C++ PRogramming Langauage".to_string(), None, vec!["Bjarne Stroustrup".to_string()], "0-201-70073-5".to_string())];
    HttpResponse::Ok().json(book_list)
}

///Adds a Book to the database from given UnidentifiedBook
///returns BookIDResponse whit new books ID
#[post("/books")]
async fn new_book(body: Json<UnidentifiedBook>) -> impl Responder {
    let book = Book::new_with_book(body.into_inner());
    // TODO: Push the new book in to the DB
    HttpResponse::Ok().json(BookIDResponse{ id: book.id })
}

#[put("/books/{id}")]
async fn update_book(path: Path<String>, body: Json<UnidentifiedBook>) -> impl Responder {
    let id = path.into_inner();
    let unwrapped_book = body.into_inner();
    let updated_book = Book { id, title: unwrapped_book.title, subtitle: unwrapped_book.subtitle,
                              authors: unwrapped_book.authors, isbn: unwrapped_book.isbn };
    // TODO: Actually update the book in the database
    HttpResponse::Ok().json(BookIDResponse { id: updated_book.id })
}

#[get("/books/{id}")]
async fn get_book_for_id(path: Path<String>) -> impl Responder {
    let id = path.into_inner();
    let book = Book { id, title: "The C++ PRogramming Langauage".to_string(), 
                      subtitle: None, authors: vec!["Bjarne Stroustrup".to_string()], isbn: "0-201-70073-5".to_string() };
    HttpResponse::Ok().json(book)
}

#[post("/books/{id}/checkout")]
async fn checkout_book_for_id(path: Path<String>, body: Json<BookStatusUpdate>) -> impl Responder {
    let id =  path.into_inner();
    let status = body.into_inner().status;
    //TODO: Update value in the Database associated with this book id
    HttpResponse::Ok()
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


#[cfg(test)]
mod book_tests {
    use super::*;

    #[actix_web::test]
    async fn book_list_test() {
    }
}
