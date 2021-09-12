# Apex Legends API (in Rust)

This package utilizes the Apex Legends Status (https://apexlegendsstatus.com) API.

Usage Example:

```rust
use apex_legends;

#[tokio::main]
async fn main() {
    match apex_legends::get_user("HeyImLifeline".to_string(), "your_api_key").await {
        Ok(data) => println!("You are level {}.", data.global.level),
        Err(e) => {
            println!("there was an error!: {}", e)
        }
    }
}
```

I have no affiliation with Apex Legends, EA, or Apex Legends Status.
