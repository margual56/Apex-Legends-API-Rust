use reqwest::{self, header::HeaderMap};

pub async fn get_request(url: String) -> Result<String, (reqwest::Error, Option<HeaderMap>)> {
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => {
            let headers = res.headers().clone();

            match res.error_for_status() {
                Ok(r) => Ok(r
                    .text()
                    .await
                    .expect("Could not retrieve text from the successful request")),
                Err(e) => Err((e, Some(headers))), //Err((, Some(headers))),
            }
        }
        Err(e) => Err((e, None)),
    }
}

#[allow(dead_code)]
pub async fn post_request(url: &str, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    match client.post(url).body(body).send().await?.text().await {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}
