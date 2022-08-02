#[cfg(test)]
mod tests {
    use std::env;

    #[tokio::test]
    async fn main() {
        dotenv::dotenv().expect("Could not load .env file");

        let user_name = env::var("USERNAME").expect("Expected key USERNAME");
        let api_key = env::var("API_KEY").expect("Expected key API_KEY");

        match apex_legends::get_user_retry(String::from(&user_name), &api_key, true).await {
            Ok(data) => {
                println!(
                    "You are level {}, and you have {} kills.",
                    data.global.level, data.stats.br_kills.value
                );

                assert!(true)
            }
            Err(e) => {
                println!("there was an error!: {}", e);

                assert!(false)
            }
        }

        match apex_legends::get_user_retry(user_name, &api_key, true).await {
            Ok(data) => {
                println!(
                    "You are level {}, and you have {} kills.",
                    data.global.level, data.stats.br_kills.value
                );

                assert!(true)
            }
            Err(e) => {
                println!("there was an error!: {}", e);

                assert!(false)
            }
        }
    }
}
