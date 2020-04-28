mod url;
mod file;
mod highlight;

use warp::{Filter};

#[tokio::main]
async fn main() {
    let urls = url::Urls::new();

    let homepage = warp::path::end().and(warp::fs::file("./html/index.html"));

    let post = warp::post()
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 1024)) // 1MB limit
        .and(warp::header::<String>("Host"))
        .and(warp::body::bytes())
        .map(move |host: String, bytes: bytes::Bytes| {
            let url = urls.next();
            let content = std::str::from_utf8(&bytes)
                .unwrap_or_else(|_| "pls provide valid utf8");
            match file::write(&format!("./data/{}", url), content) {
                // TODO: detect protocol
                Ok(_) => format!("https://{}/{}", host, url),
                Err(error) => error.to_string(),
            }
        });

    let pasta = warp::path!(String).map(|filename: String| {
        let file = file::read(&format!("./data/{}", file::basename(&filename)));
        let html = match file {
            Ok(content) => highlight::highlight(&content, &filename),
            Err(error) => error.to_string().to_lowercase(),
        };
        warp::reply::html(html)
    });

    let routes = warp::any().and(pasta.or(post).or(homepage));

    warp::serve(routes).run(([0, 0, 0, 0], 8001)).await;
}
