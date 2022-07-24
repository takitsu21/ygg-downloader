
use futures::executor::block_on;
use reqwest::{Client, Response, StatusCode};


trait YggScrapper {
    fn get_data(&self) -> String;
    fn send_file(&self, local_path: &str, remote_path: &str, remote_host: &str);
    fn new() -> YggScrapperImpl;
}

struct YggScrapperImpl {
    filename: Option<String>,
    query: Option<String>,
    result: Option<i64>,
    order: Option<String>,
    sort_by: Option<String>,
}

impl YggScrapper for YggScrapperImpl {
    fn get_data(&self) -> String {
        return self.query.as_deref().unwrap_or("").to_string();
    }

    fn send_file(&self, local_path: &str, remote_path: &str, remote_host: &str) {
        println!("Sending file {} to {} => {}", local_path, remote_path, remote_host);
    }

    fn new() -> YggScrapperImpl {
        YggScrapperImpl {
            filename: Some("".to_string()),
            query: Some("".to_string()),
            result: Some(10),
            order: Some("desc".to_string()),
            sort_by: Some("completed".to_string()),
        }
    }
}
fn main() {
    let scrapper = YggScrapperImpl::new();
    println!("{}", scrapper.get_data());

    let future = block_on(async_request());
    println!("{:?}", future);
}

pub async fn async_request() -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = match client.post("http://httpbin.org/post")
        .body("the exact body that is sent")
        .send()
        .await {
            Ok(res) => res,
            Err(e) => return Err(e),
        };

    return Ok(res);
}