# Apex Legends API (in Rust)

This package utilizes the Apex Legends Status (https://apexlegendsstatus.com) API.

Usage Example:

```rust
use apex_legends;

#[tokio::main]
async fn main() {
    match apex_legends::get_user_retry("HeyImLifeline".to_string(), "your_api_key", true).await {
        Ok(data) => println!("You are level {}.", data.global.level),
        Err(e) => println!("There was an error!: {}", e)
    }
}
```

I have no affiliation with Apex Legends, EA, or Apex Legends Status.

## A note about the failing test

This is a known issue in the API. It has a rate limit, so it should return code 429 when the limit is reached. Instead, it returns 200 OK, so the library immediately retries and, unsurprisingly, it fails.


# Authors

The original author is [KasprDev](https://github.com/KasprDev), and this is a fork with some improvements for stability and extra features. Some of them were upstreamed, but not all of them as of yet.