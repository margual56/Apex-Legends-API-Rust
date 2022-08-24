use std::time::Duration;

use reqwest::{header::HeaderMap, StatusCode};

pub mod data_types;
mod http;

/// Default time to wait after a 429 error code
pub const DEFAULT_RATE: f32 = 3.0;

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

/// Given an optional header, return the value of the header if it exists or DEFAULT_RATE
/// The header is `x-current-rate`
fn get_rate(header: Option<HeaderMap>) -> f32 {
    if let Some(h) = header {
        if let Some(value) = h.get("x-current-rate") {
            let default = DEFAULT_RATE.to_string();
            let string_value = value.to_str().unwrap_or(default.as_str());

            return string_value.parse().unwrap_or(DEFAULT_RATE);
        } else {
            return DEFAULT_RATE;
        }
    } else {
        return DEFAULT_RATE;
    }
}

/// Gets information about a User. This version automatically retries if the API returns code 429 (too many requests).
/// For the sleep time it reads the `x-current-rate` header or uses the DEFAULT_RATE
/// See https://apexlegendsapi.com/#player-statistics
///
/// # Arguments
///
/// * `username` - The Origin username of the player
/// * `api_key` - The API key for https://apexlegendsstatus.com
/// * `retry` - Wether to retry after a timeout or error out immediately
///
/// # Examples
/// ```
/// let user_name = env::var("USERNAME").expect("Expected key USERNAME");
/// let api_key = env::var("API_KEY").expect("Expected key API_KEY");
/// 
/// // This example automatically handles the 429 error code (too many requests)
/// match apex_legends::get_user_retry(String::from(&user_name), &api_key, true).await {
///    Ok(data) => {
///        println!(
///            "You are level {}, and you have {} kills.",
///            data.global.level, data.stats.br_kills.value
///        );
///    }
///    Err(e) => {
///        println!("there was an error!: {}", e);
///    }
/// }
/// ```
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
            if let Some(code) = e.0.status() {
                if code == StatusCode::TOO_MANY_REQUESTS && retry {
                    let time = get_rate(e.1);

                    tokio::time::sleep(Duration::from_secs_f32(time)).await;

                    get_user(username, api_key).await
                } else {
                    handle_error::<data_types::ApexUser>(code)
                }
            } else {
                Err(format!("{}", e.0))
            }
        }
    }
}

/// Gets information about a User. This version does not handle code 429 (too many requests)
/// See https://apexlegendsapi.com/#player-statistics
///
/// # Arguments
///
/// * `username` - The Origin username of the player
/// * `api_key` - The API key for https://apexlegendsstatus.com
///
/// # Examples
/// ```
/// let user_name = env::var("USERNAME").expect("Expected key USERNAME");
/// let api_key = env::var("API_KEY").expect("Expected key API_KEY");
/// 
/// // This example will fail if the API returns the 429 error code (too many requests)
/// match apex_legends::get_user(String::from(&user_name), &api_key).await {
///    Ok(data) => {
///        println!(
///            "You are level {}, and you have {} kills.",
///            data.global.level, data.stats.br_kills.value
///        );
///    }
///    Err(e) => {
///        println!("there was an error!: {}", e);
///    }
/// }
/// ```
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
            if let Some(code) = e.0.status() {
                handle_error::<data_types::ApexUser>(code)
            } else {
                Err(format!("{}", e.0))
            }
        }
    }
}

/// Gets information about the recent games.
/// You must be whitelisted to use this API. It has a strict limit of 5 uniques players queried per hour.
/// See https://apexlegendsapi.com/#match-history
///
/// * `user_id` - The player's UID
/// * `api_key` - The API key for https://apexlegendsstatus.com
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
            if let Some(code) = e.0.status() {
                handle_error::<Vec<data_types::ApexGame>>(code)
            } else {
                Err("There was an error getting your recent matches.".to_string())
            }
        }
    }
}

/// Returns a player's UID from a given name, but also works with Playstation and Xbox players
/// See https://apexlegendsapi.com/#name-to-uid
///
/// ## Warning
/// This function does not work as intended:
///    It will not retry because the API returns 200 OK instead of code 429,
///    so there is no way to check if error 429 has occurred
///
/// # Parameters
/// * `user_id` - The player's UID
/// * `api_key` - The API key for https://apexlegendsstatus.com
/// * `retry` - Wether to retry after a timeout or error out immediately
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
            Err(e) => match serde_json::from_str::<data_types::ApexError>(&resp) {
                Ok(err) => Err(err.message),
                Err(_) => Err(format!("{}", e)),
            },
        },
        Err(e) => {
            if let Some(code) = e.0.status() {
                if code == StatusCode::TOO_MANY_REQUESTS && retry {
                    tokio::time::sleep(Duration::from_secs_f32(get_rate(e.1))).await;

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
            Err(e) => match serde_json::from_str::<data_types::ApexError>(&resp) {
                Ok(err) => Err(err.message),
                Err(_) => Err(format!("{}", e)),
            },
        },
        Err(e) => {
            if let Some(code) = e.0.status() {
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
            if let Some(code) = e.0.status() {
                if code == StatusCode::TOO_MANY_REQUESTS && retry {
                    tokio::time::sleep(Duration::from_secs_f32(get_rate(e.1))).await;

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
            if let Some(code) = e.0.status() {
                handle_error::<data_types::ApexMapRotation>(code)
            } else {
                Err("There was an error getting the map rotation.".to_string())
            }
        }
    }
}
