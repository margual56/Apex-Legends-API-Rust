use reqwest;

pub async fn get_request(url: String) -> Result<String, reqwest::Error> {
    let r = reqwest::get(url).await?.text().await?;
    println!("{}", r);
    Ok(r)
}

pub async fn post_request(url: &str, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let r = client.post(url).body(body).send().await?.text().await?;

    Ok(r)
}