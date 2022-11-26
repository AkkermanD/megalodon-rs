use megalodon::{entities, error, generator};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL!!");
        return
    };
    let Ok(token) = env::var("MASTODON_ACCESS_TOKEN") else {
        println!("Specify MASTODON_ACCESS_TOKEN!!");
        return
    };

    let file_path = "./sample.jpg".to_string();
    let res = upload_media(url.as_str(), token.to_owned(), file_path).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn upload_media(
    url: &str,
    access_token: String,
    file_path: String,
) -> Result<entities::Attachment, error::Error> {
    let client = generator(
        megalodon::SNS::Mastodon,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.upload_media(file_path, None).await?;
    Ok(res.json())
}
