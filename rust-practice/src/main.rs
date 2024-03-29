#![deny(clippy::all)]

use futures::executor::block_on;
use tokio::time::{sleep, Duration};

async fn call_api_one() -> String {
    sleep(Duration::from_secs(1)).await;
    "Saiteja1".to_string()
}

async fn call_api_two() -> String {
    sleep(Duration::from_secs(1)).await;
    "Saiteja2".to_string()
}

#[tokio::main]
async fn main() {
    let name1 = call_api_one();
    println!("Hello {}", name1.await);

    let name2 = call_api_two().await;
    println!("Hello {}", name2);
}
