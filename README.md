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

# Authors

The original author is [KasprDev](https://github.com/KasprDev), and this is a fork with some improvements for stability and extra features. Some of them were upstreamed, but not all of them as of yet.