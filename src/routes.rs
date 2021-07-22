// use markdown_storage::parse_markdown;
use actix_web::{
    get, HttpRequest, HttpResponse
};
use mime_db;
// use actix_files::NamedFile;
use rust_embed::RustEmbed;
use std::str;

#[derive(RustEmbed)]
#[folder = "./web/dist"]
struct Asset;

fn get_asset(filename: &str) -> &[u8] {
    match Asset::get(filename) {
        Some(content) => {
            &*content
        },
        None => {
            &[]
        }
    }
}

#[get("/{filename:.*}")]
pub async fn web_ui(req: HttpRequest) -> HttpResponse {
    let filename = req.match_info().query("filename");
    let mime_type = mime_db::lookup(filename).unwrap_or("plain/text");
    let bytes = get_asset(filename);
    HttpResponse::Ok()
        .content_type(mime_type)
        .body(bytes)

    // if filename.len() == 0 {
    //     Ok(NamedFile::open("./web/dist/index.html")?)
    // } else {
    //     Ok(NamedFile::open(format!("./web/dist/{}", filename))?)
    // }
}
