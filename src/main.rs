use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use reqwest::{Client, Error};
use scraper::{Html, Selector};
use teloxide_core::prelude::*;
use teloxide_core::types::ParseMode;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let recipient = "@ComuneVareseOrdinanze".to_string();

    let source_url = Url::parse("https://www.comune.varese.it/c012133/zf/index.php/servizi-aggiuntivi/index/index/idtesto/565").unwrap();
    let base_url = "http://www.comune.varese.it";

    let res = Client::default().get(source_url);
    let body = res.send().await?;

    let fragment = Html::parse_document(body.text().await?.as_str());
    let selector = Selector::parse("#main > div > div.Grid.Grid--withGutter > div > ul > li > a").unwrap();

    let bot = Bot::from_env().parse_mode(ParseMode::Html);

    let mut file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open("notified.db")
        .unwrap();

    let reader = BufReader::new(file.try_clone().unwrap());
    let lines = reader.lines().into_iter().map(|x| {x.unwrap()}).collect::<Vec<_>>();

    for x in fragment.select(&selector) {
        let text = x.text().collect::<Vec<_>>().join(" ");
        let href = x.value().attr("href").unwrap_or_default();

        let result = Url::parse(href);
        let url = match result {
            Ok(url) => url,
            Err(_) => Url::parse(format!("{}{}", base_url, href).as_str()).unwrap()
        };

        let mut found = false;
        for line in lines.clone() {
            if line.eq(url.as_str()) {
                found = true;
            }
        }

        if !found {
            let body = format!("<b>Nuova Ordinanza Comune di Varese</b>\n\n<a href=\"{}\">{}</a>", url.as_str(), text);
            bot.send_message(recipient.clone(), body).await.unwrap();

            if let Err(e) = writeln!(file, "{}", url.as_str()) {
                eprintln!("Couldn't write to file: {}", e);
            }

            println!("Sending {}", url.as_str());
        }
    }

    Ok(())
}
