// cargo run --bin 80-basic-async-program -- https://www.rust-lang.org

use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;
    //
    // or you can chains of method like this
    let response_text = trpl::get(url).await.text().await;

    /*
    the trpl::get function to fetch whatever URL is passed in and add the await keyword to await the response. To get the text of the response, we call its text method, and once again await it with the await keyword. Both of these steps are asynchronous. For the get function, we have to wait for the server to send back the first part of its response (include HTTP headers, cookies, and so on) and can be delivered separately from the response body. Especially if the body is very large, it can take some time for it all to arrive. Because we have to wait for the entirety of the response to arrive, the text method is also async. */

    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];

        // page_title started here because we have awaited them, (remember futures are lazy).
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
