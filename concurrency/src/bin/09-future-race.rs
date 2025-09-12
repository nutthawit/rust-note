// cargo run --bin 9-future-either -- https://www.rust-lang.org https://stackoverflow.com

use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());

    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);
        // Remember, these don’t do anything yet, because futures are lazy and we haven’t yet awaited them.

        // We pass the futures to trpl::race, which returns a value to indicate which of the futures passed to it finishes first.
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await /* future started here */  {
                Either::Left(left) => left,
                Either::Right(right) => right,
        };

        /* Note: Under the hood, race is built on a more general function, select, which you will encounter more often in real-world Rust code. A select function can do a lot of things that the trpl::race function can’t, but it also has some additional complexity that we can skip over for now. */

        // trpl::Either == futures::Either
        //
        // The Either type is somewhat similar to a Result in that it has two cases. Unlike Result, though, there is no notion of success or failure baked into Either. Instead, it uses Left and Right to indicate “one or the other”.
        println!("{url} return first");
        match maybe_title {
            Some(title) => println!("Its page title is '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}
