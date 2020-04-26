mod url;
mod file;

use warp::{http::Uri, Filter};
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
                    window.location.href = (
                        await fetch('/', { method: 'POST', body: textarea.value })
                    ).url;
                });
        </script>
    "#));


    let post = warp::post()
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 1024)) // 1MB limit
        .and(warp::body::bytes())
        .map(move |bytes: bytes::Bytes| {
            let url = hash.lock().unwrap().next();
            // let mut buffer = String::new();
            // bytes.read_to_string(&mut buffer);
            // println!("{:#?}", bytes.to_string());
            bytes.into_iter().collect::<String>();
            warp::redirect::redirect(Uri::from_static("/asdf"))
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
