use hyper::StatusCode;
use hyper::{ Client, Method, Body, Request };
use serde_json;
use serde::Serialize;
use serde::Deserialize;
use hyper_tls::HttpsConnector;
use colored::Colorize;

use super::ext_consts::API_KEY; // this wan kill me (`..`)
use super::ext_consts::GPT_API_URL;

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>, // learinig this , a vector is a fancy array ds seen in c++
    pub usage: Usage,
}
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

// messageBody
#[derive(Serialize)]
pub struct ChatInput {
    pub model: String,
    pub messages: Vec<Message>,
}
#[derive(Serialize, Debug, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
 
#[derive(Debug, serde::Deserialize)]
struct ChatErrorResponse {
    error: ErrorDetails,
}

#[derive(Debug, serde::Deserialize)]
struct ErrorDetails {
    message: String,
    r#type: String,
    param: Option<serde_json::Value>,
    code: String,
}

#[tokio::main]
pub async fn make_prompt(
    prompt: &str
) -> Result<Option<ChatResponse>, Box<dyn std::error::Error + Send + Sync>> {
    let chat = ChatInput {
        model: String::from("gpt-3.5-turbo"),
        messages: vec![Message {
            role: String::from("user"),
            content: String::from(prompt),
        }],
    };

    let https = HttpsConnector::new();

    let json_string = serde_json::to_string(&chat).unwrap();

    let req = Request::builder()
        .method(Method::POST)
        .uri(GPT_API_URL)
        .header("content-type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .body(Body::from(json_string))?;

    // let req = ...
    let client = Client::builder().build(https);

    // POST it...
    let resp = client.request(req).await?;

    // println!("{}",.to_string());
    if resp.status() == StatusCode::OK {
        // Read the response body as bytes
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

        let response_string = String::from_utf8(body_bytes.to_vec())?;

        let responce_json: ChatResponse = serde_json::from_str(&response_string)?;

        Ok(Some(responce_json))

    } else {
        let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let response_string = String::from_utf8(body_bytes.to_vec())?;
        let responce_json: ChatErrorResponse = serde_json::from_str(&response_string)?;
 
        println!("{}", responce_json.error.message.red().to_string());
        Ok(None)
    }
}

fn save_prompt_history(prompt: &str) {
    //do some work with files
}

// init with the api
// get responce toake and use that token to make request
