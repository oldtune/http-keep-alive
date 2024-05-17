#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    loop {
        let _ = client
            .get("http://localhost:8000/")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
    }
}
