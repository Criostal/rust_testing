
use reqwest;
use reqwest::header::AUTHORIZATION;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;

#[tokio::main]
async fn main() {
    #![allow(non_snake_case)]
    let client = reqwest::Client::new();

    let buildId = 31706;
    let batchSize = 50;
    let url = format!("http://ds-vm-csovcs01:8080/NxCollection/Medication/_apis/build/builds/{buildId}/workitems?$top={batchSize}&api-version=5.0");
    let token = "emrkd7xwylje5daa4sy5w2dh2y4wzfmqacvtm4ybqpmk3ag6426q";
    let token2 = "OmFmdGV3aXhvY3R3MmVzZ3I1YW1tazZkYzY1aGthdWYzaGo3ZDd2YjdvNnpnc3dibWlmN3E=";

    

    let response = client
    .get(url)
    .header(AUTHORIZATION, format!("Basic {token2}"))
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .send()
    .await
    .unwrap()
    .text()
    .await;

    println!("{:?}", response)
    
}
