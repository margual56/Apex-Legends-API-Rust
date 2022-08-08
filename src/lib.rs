use std::time::Duration;

use reqwest::StatusCode;

mod data_types;
mod http;

/// Time (in seconds) to wait after the error TOO_MANY_REQUESTS (429)
pub const WAIT_TIME: f32 = 2.5;

/// Simple function to standarize error messages given a Status Code
fn handle_error<T>(status: StatusCode) -> Result<T, String> {
    // Documentation: https://apexlegendsapi.com/#errors
    match status {
        StatusCode::TOO_MANY_REQUESTS => Err(String::from(
            "Too many requests: wait 1 or 2 seconds please",
        )),
        StatusCode::UNAUTHORIZED => Err(String::from(
            "The API key is incorrect, please contact the bot administrator",
        )),
        StatusCode::NOT_FOUND => Err(String::from("Either the apexlegendsapi.com")),
        StatusCode::INTERNAL_SERVER_ERROR => {
            Err(String::from("There was an internal server error"))
        }
        _ => Err(format!("{}", status)),
    }
}

pub async fn get_user_retry(
    username: String,
    api_key: &str,
    retry: bool,
) -> Result<data_types::ApexUser, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/bridge?version=5&platform=PC&player={}&auth={}",
        username, api_key
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                if code == StatusCode::TOO_MANY_REQUESTS && retry {
                    tokio::time::sleep(Duration::from_secs_f32(WAIT_TIME)).await;

                    get_user(username, api_key).await
                } else {
                    handle_error::<data_types::ApexUser>(code)
                }
            } else {
                Err(format!("{}", e))
            }
        }
    }
}

pub async fn get_user(username: String, api_key: &str) -> Result<data_types::ApexUser, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/bridge?version=5&platform=PC&player={}&auth={}",
        username, api_key
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                handle_error::<data_types::ApexUser>(code)
            } else {
                Err(format!("{}", e))
            }
        }
    }
}

pub async fn get_recent_games_retry(
    user_id: String,
    api_key: &str,
    retry: bool,
) -> Result<Vec<data_types::ApexGame>, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/games?auth={}&uid={}",
        api_key, user_id
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                if code == StatusCode::TOO_MANY_REQUESTS && retry {
                    tokio::time::sleep(Duration::from_secs_f32(WAIT_TIME)).await;

                    get_recent_games(user_id, api_key).await
                } else {
                    handle_error::<Vec<data_types::ApexGame>>(code)
                }
            } else {
                Err("There was an error getting your recent matches.".to_string())
            }
        }
    }
}
pub async fn get_recent_games(
    user_id: String,
    api_key: &str,
) -> Result<Vec<data_types::ApexGame>, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/games?auth={}&uid={}",
        api_key, user_id
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                handle_error::<Vec<data_types::ApexGame>>(code)
            } else {
                Err("There was an error getting your recent matches.".to_string())
            }
        }
    }
}

pub async fn get_uid_from_username_retry(
    username: String,
    api_key: &str,
    retry: bool,
) -> Result<data_types::ApexProfile, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/nametouid?player={}&platform=PC&auth={}",
        username, api_key
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                if code == StatusCode::TOO_MANY_REQUESTS && retry {
                    tokio::time::sleep(Duration::from_secs_f32(WAIT_TIME)).await;

                    get_uid_from_username(username, api_key).await
                } else {
                    handle_error::<data_types::ApexProfile>(code)
                }
            } else {
                Err("There was an error getting the user id.".to_string())
            }
        }
    }
}

pub async fn get_uid_from_username(
    username: String,
    api_key: &str,
) -> Result<data_types::ApexProfile, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/nametouid?player={}&platform=PC&auth={}",
        username, api_key
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("{}", e)),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                handle_error::<data_types::ApexProfile>(code)
            } else {
                Err("There was an error getting the user id.".to_string())
            }
        }
    }
}

pub async fn get_map_rotation_retry(
    api_key: &str,
    retry: bool,
) -> Result<data_types::ApexMapRotation, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/maprotation?version=2&auth={}",
        api_key
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(_) => Err("Unable to deserialize JSON.".to_string()),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                if code == StatusCode::TOO_MANY_REQUESTS && retry {
                    tokio::time::sleep(Duration::from_secs_f32(WAIT_TIME)).await;

                    get_map_rotation(api_key).await
                } else {
                    handle_error::<data_types::ApexMapRotation>(code)
                }
            } else {
                Err("There was an error getting the map rotation.".to_string())
            }
        }
    }
}

pub async fn get_map_rotation(api_key: &str) -> Result<data_types::ApexMapRotation, String> {
    match http::get_request(format!(
        "https://api.mozambiquehe.re/maprotation?version=2&auth={}",
        api_key
    ))
    .await
    {
        Ok(resp) => match serde_json::from_str(&resp) {
            Ok(data) => Ok(data),
            Err(_) => Err("Unable to deserialize JSON.".to_string()),
        },
        Err(e) => {
            if let Some(code) = e.status() {
                handle_error::<data_types::ApexMapRotation>(code)
            } else {
                Err("There was an error getting the map rotation.".to_string())
            }
        }
    }
}
