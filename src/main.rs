use reqwest::{Client, Response, Error};
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // https://www.comune.varese.it/c012133/zf/index.php/servizi-aggiuntivi/index/index/idtesto/565
    let res = Client::default().get("https://www.comune.varese.it/c012133/zf/index.php/servizi-aggiuntivi/index/index/idtesto/565");
    let body = res.send().await?;

    let fragment = Html::parse_document(body.text().await?.as_str());
    let selector = Selector::parse("#main > div > div.Grid.Grid--withGutter > div > ul > li > a").unwrap();

    for element in fragment.select(&selector) {
        let text: String = element.text().collect::<Vec<_>>().join(" ");
        let url = element.value().attr("href").unwrap_or_default();
        println!("{:?}", text);
        println!("{:?}", url);
    }

    Ok(())
}
