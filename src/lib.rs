mod http;
mod data_types;

pub async fn get_user(username: String, api_key: &str) -> Result<data_types::ApexUser, String> {
    match http::get_request(format!("https://api.mozambiquehe.re/bridge?version=5&platform=PC&player={}&auth={}", username, api_key)).await {
        Ok(resp) => {
            match serde_json::from_str(&resp) {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("{}", e))
            }
        },
        Err(_) => Err("There was an error getting your profile.".to_string())
    }
}

pub async fn get_recent_games(user_id: String, api_key: &str) -> Result<Vec<data_types::ApexGame>, String> {
    match http::get_request(format!("https://api.mozambiquehe.re/games?auth={}&uid={}", api_key, user_id)).await {
        Ok(resp) => {
            match serde_json::from_str(&resp) {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("{}", e))
            }
        },
        Err(_) => Err("There was an error getting your recent matches.".to_string())
    }
}

pub async fn get_uid_from_username(username: String, api_key: &str) -> Result<data_types::ApexProfile, String> {
    match http::get_request(format!("https://api.mozambiquehe.re/nametouid?player={}&platform=PC&auth={}", username, api_key)).await {
        Ok(resp) => {
            match serde_json::from_str(&resp) {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("{}", e))
            }
        },
        Err(_) => Err("There was an error getting the user id.".to_string())
    }
}

pub async fn get_map_rotation(api_key: &str) -> Result<data_types::ApexMapRotation, String> {
    match http::get_request(format!("https://api.mozambiquehe.re/maprotation?version=2&auth={}", api_key)).await {
        Ok(resp) => {
            match serde_json::from_str(&resp) {
                Ok(data) => Ok(data),
                Err(_) => Err("Unable to deserialize JSON.".to_string())
            }
        },
        Err(_) => Err("There was an error getting the map rotation.".to_string())
    }
}