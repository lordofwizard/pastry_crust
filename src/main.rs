use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::sync::Mutex;
use actix_files::NamedFile;
struct AppState {
    db: Mutex<Connection>,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("index.html"))
}

async fn submit(content: web::Form<FormData>, data: web::Data<AppState>) -> impl Responder {
    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let conn = data.db.lock().unwrap();
    conn.execute(
        "INSERT INTO pastes (token, content) VALUES (?, ?)",
        params![&token, &content.content],
    )
    .expect("Failed to insert into database");

    HttpResponse::SeeOther()
        .header("Location", format!("/paste/{}", token))
        .finish()
}

async fn get_paste(token: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let content = conn
        .query_row(
            "SELECT content FROM pastes WHERE token = ?",
            params![token.to_string()],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "Paste not found".to_string());

    HttpResponse::Ok().body(format!("<pre>{}</pre>", content))
}

#[derive(serde::Deserialize)]
struct FormData {
    content: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Connection::open("pastes.db").expect("Failed to open database");
    db.execute(
        "CREATE TABLE IF NOT EXISTS pastes (token TEXT PRIMARY KEY, content TEXT)",
        params![],
    )
    .expect("Failed to create table");

    let app_state = web::Data::new(AppState {
        db: Mutex::new(db),
    });


    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/style.css").to(|| {
                async { NamedFile::open("src/style.css") }
            }))
            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
            .route("/paste/{token}", web::get().to(get_paste))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
