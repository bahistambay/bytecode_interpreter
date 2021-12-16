use std::fs::File;
use std::io::Write;
use std::io::{prelude::*, BufReader};

use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use futures_util::TryStreamExt as _;
use serde_json;
use uuid::Uuid;

mod interpreter;
mod op;
use op::ByteCode;

async fn save_file_and_interpret(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut return_value = HttpResponse::Ok().body("ByteCodeParseError");
    let mut stack = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;

        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |f| sanitize_filename::sanitize(f),
        );
        let filepath = format!("./tmp/{}", filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
        }
        let file = File::open(String::from(format!("./tmp/{}", filename)))?;
        let reader = BufReader::new(file);
        let lines_iter = reader.lines().map(|l| l.unwrap());
        let mut bytecode_list = Vec::new();

        for line in lines_iter {
            let s: &str = &line;
            let form_s = format!("\"{}\"", s);
            //deserialize form_s to ByteCode type
            bytecode_list.push(serde_json::from_str::<ByteCode>(&form_s).unwrap());
        }
        return_value = HttpResponse::Ok().json(interpreter::interpret(
            &mut stack,
            &mut bytecode_list,
        ))
    }

    Ok(return_value)
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::fs::create_dir_all("./tmp")?;
    env_logger::init();

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(save_file_and_interpret)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
