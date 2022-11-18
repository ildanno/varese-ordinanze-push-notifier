use std::env;
use reqwest::{Client, Response, Error};
use scraper::{Html, Selector};
use teloxide_core::prelude::*;
use teloxide_core::types::ForwardedFrom::Chat;
use teloxide_core::types::ParseMode;
use teloxide_core::types::Recipient::Id;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let chat_id = env::var("CHAT_ID").unwrap().parse::<u64>().unwrap();

    // https://www.comune.varese.it/c012133/zf/index.php/servizi-aggiuntivi/index/index/idtesto/565
    let res = Client::default().get("https://www.comune.varese.it/c012133/zf/index.php/servizi-aggiuntivi/index/index/idtesto/565");
    let body = res.send().await?;

    let fragment = Html::parse_document(body.text().await?.as_str());
    let selector = Selector::parse("#main > div > div.Grid.Grid--withGutter > div > ul > li > a").unwrap();

    let bot = Bot::from_env().parse_mode(ParseMode::Html);

    for x in fragment.select(&selector) {
        let text = x.text().collect::<Vec<_>>().join(" ");
        let url = x.value().attr("href").unwrap_or_default();
        let body = format!("<b>Nuova Ordinanza Comune di Varese</b>\n\n<a href=\"{}\">{}</a>", url, text);
        bot.send_message(UserId(chat_id), body).await.unwrap();
    }

    Ok(())
}
