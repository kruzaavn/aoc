use reqwest::header::{HeaderMap, HeaderValue};
use cookie::Cookie;
use std::fs;
use reqwest::Error;

pub async fn get_data(year: i32, day: i32) -> Result<String, Error> {
    // Read the session key from the file
    let session_key = fs::read_to_string("/Users/aleksandrskruza/dev/aoc/session.txt")
        .expect("Unable to read session key from file");

    // Create a client
    let client = reqwest::Client::new();

    // Create custom headers
    let mut headers = HeaderMap::new();

    // Create a cookie
    let cookie = Cookie::new("session", session_key);

    // Add the cookie to the request headers
    headers.insert(
        reqwest::header::COOKIE,
        HeaderValue::from_str(&cookie.to_string())
            .expect("Invalid cookie value"),
    );

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    // Make a GET request with custom headers
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    // Check if the request was successful

    let body = response.text().await?;
    Ok(body)

}


// #[tokio::main]
// async fn main() {
//     match get_data(2020, 2).await {
//         Ok(response) => println!("Response: {}", response),
//         Err(err) => eprintln!("Error: {}", err),
//     }
// }

