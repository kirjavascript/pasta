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
        .and(warp::header::optional::<String>("X-Forwarded-Host"))
        .and(warp::header::optional::<String>("X-Forwarded-Proto"))
        .and(warp::body::bytes())
        .map(move |host: String, protocol: Option<String>, forwarded_host: Option<String>, bytes: bytes::Bytes| {
            let url = urls.next();
            let host = forwarded_host.unwrap_or(host);
            let protocol = protocol.unwrap_or_else(|| "http".to_string());
            let content = std::str::from_utf8(&bytes)
                .unwrap_or_else(|_| "pls provide valid utf8");
            match file::write(&format!("./data/{}", url), content) {
                Ok(_) => format!("{}://{}/{}", protocol, host, url),
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
