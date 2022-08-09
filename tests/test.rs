#[cfg(test)]
mod tests {
    use apex_legends::data_types;
    use std::env;

    fn print_data<T>(res: Result<T, String>, f: fn(T) -> String) {
        match res {
            Ok(data) => {
                println!("{}", f(data));

                assert!(true)
            }
            Err(e) => {
                println!("there was an error!: {}", e);

                assert!(false)
            }
        }
    }

    #[tokio::test]
    async fn user() {
        dotenv::dotenv().expect("Could not load .env file");

        let user_name = env::var("USERNAME").expect("Expected key USERNAME");
        let api_key = env::var("API_KEY").expect("Expected key API_KEY");

        print_data::<data_types::ApexUser>(
            apex_legends::get_user_retry(String::from(&user_name), &api_key, true).await,
            |data| {
                format!(
                    "You are level {}, and you have {} kills.",
                    data.global.level, data.stats.br_kills.value
                )
            },
        );

        print_data::<data_types::ApexUser>(
            apex_legends::get_user_retry(String::from(&user_name), &api_key, true).await,
            |data| {
                format!(
                    "You are level {}, and you have {} kills.",
                    data.global.level, data.stats.br_kills.value
                )
            },
        );
    }
    #[tokio::test]
    #[should_panic] // Because the API returns 200 OK when it should return 429
    async fn uid() {
        dotenv::dotenv().expect("Could not load .env file");

        let user_name = env::var("USERNAME").expect("Expected key USERNAME");
        let api_key = env::var("API_KEY").expect("Expected key API_KEY");

        print_data::<data_types::ApexProfile>(
            apex_legends::get_uid_from_username_retry(String::from(&user_name), &api_key, true)
                .await,
            |data| format!("Your UID is {}", data.uid),
        );
        print_data::<data_types::ApexProfile>(
            apex_legends::get_uid_from_username_retry(String::from(&user_name), &api_key, true)
                .await,
            |data| format!("Your UID is {}", data.uid),
        );
    }

    #[tokio::test]
    async fn map_rotation() {
        dotenv::dotenv().expect("Could not load .env file");

        let api_key = env::var("API_KEY").expect("Expected key API_KEY");

        print_data::<data_types::ApexMapRotation>(
            apex_legends::get_map_rotation_retry(&api_key, true).await,
            |data| {
                format!(
                    "The current ranked map is {}",
                    data.arenas_ranked.current.map
                )
            },
        );
        print_data::<data_types::ApexMapRotation>(
            apex_legends::get_map_rotation_retry(&api_key, true).await,
            |data| {
                format!(
                    "The current ranked map is {}",
                    data.arenas_ranked.current.map
                )
            },
        );
    }
}
