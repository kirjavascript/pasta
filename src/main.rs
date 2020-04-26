mod url;
mod file;

use warp::{Filter};
use std::sync::Arc;
use std::sync::Mutex;

#[tokio::main]
async fn main() {
    let hash = Arc::new(Mutex::new(url::UrlHash::new(1)));

    let homepage = warp::path::end().map(|| warp::reply::html(r#"
        <textarea></textarea>
        <button type="button">Submit</button>
        <script>
            const textarea = document.querySelector('textarea');
            document.querySelector('button')
                .addEventListener('click', async () => {
                    window.location.href = await (
                        await fetch('/', { method: 'POST', body: textarea.value })
                    ).text();
                });
        </script>
    "#));

    let post = warp::post()
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 1024)) // 1MB limit
        .and(warp::body::bytes())
        .map(move |bytes: bytes::Bytes| {
            let url = hash.lock().unwrap().next();
            let content = std::str::from_utf8(&bytes).unwrap();
            file::write(&format!("./data/{}", url), content).unwrap();
            format!("/{}", url)
        });

    let pasta = warp::path!(String).map(|s| {
        let html = match file::read(&format!("./data/{}", s)) {
            Ok(contents) => format!("file: <pre>{}</pre>", contents),
            Err(error) => format!("error: {}", error),
        };
        warp::reply::html(html)
    });

    let routes = warp::any()
        .and(pasta.or(post).or(homepage));

    warp::serve(routes).run(([127, 0, 0, 1], 8001)).await;
}
