mod url;
mod file;

use warp::{Filter};

#[tokio::main]
async fn main() {
    let urls = url::Urls::new();

    let homepage = warp::path::end().map(|| warp::reply::html(r#"
        <textarea></textarea>
        <button type="button">Submit</button>
        <pre>curl -X POST --data "some text" <span></span></pre>
        <script>
            const textarea = document.querySelector('textarea');
            document.querySelector('button')
                .addEventListener('click', async () => {
                    window.location.href = await (
                        await fetch('/', { method: 'POST', body: textarea.value })
                    ).text();
                });
            document.querySelector('span').textContent = location.href;
        </script>
    "#));

    let post = warp::post()
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024 * 1024)) // 1MB limit
        .and(warp::body::bytes())
        .map(move |bytes: bytes::Bytes| {
            let url = urls.next();
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
