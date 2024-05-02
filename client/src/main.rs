#[tokio::main]
async fn main() {
    loop {
        let result = reqwest::get("http://localhost:8000/")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
    }
}
