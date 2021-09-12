use reqwest;

pub async fn get_request(url: String) -> Result<String, reqwest::Error> {
    match reqwest::get(url).await?.text().await {
        Ok(data) => Ok(data),
        Err(e) => Err(e)
    }
}

pub async fn post_request(url: &str, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    match client.post(url).body(body).send().await?.text().await {
        Ok(data) => Ok(data),
        Err(e) => Err(e)
    }
}