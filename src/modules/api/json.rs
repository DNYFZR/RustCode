// JSON API Handler 

use reqwest::blocking::{Client, Response};
use reqwest::Error;
use serde_json;

pub fn get(url: &str, header_key:&str, header_value:&str,) -> Result<serde_json::Value, reqwest::Error>{
    let cli = Client::new();
    let res: serde_json::Value = cli
        .get(url)
        .header(header_key, header_value)
        .send()?
        .json()?;
    
    Ok(res)
}

pub fn post(url: &str, header_key:&str, header_value:&str, body: String) -> Result<Response, Error>{
    let cli = Client::new();
    let res = cli
        .post(url)
        .header(header_key, header_value)
        .body(body)
        .send()?;
    
    Ok(res)
}

pub fn put(url: &str, header_key:&str, header_value:&str, body: String) -> Result<Response, Error>{
    let cli = Client::new();
    let res = cli
        .put(url)
        .header(header_key, header_value)
        .body(body)
        .send()?;
    
    Ok(res)
}

pub fn delete(url: &str, header_key:&str, header_value:&str, body: String) -> Result<Response, Error>{
    let cli = Client::new();
    let res = cli
        .delete(url)
        .header(header_key, header_value)
        .body(body)
        .send()?;
    
    Ok(res)
}

// Tests
#[cfg(test)]
mod test_json {
    use super::*;
    use serde_json;

    #[test]
    fn test_get() {
        let base_url = "https://dummyjson.com/products/";
        let header_key = "Content-Type"; 
        let header_value = "application/json";
        let product_endpoint = "1";
    
        let url = format!("{}{}", base_url, product_endpoint);
    
        // Get reqest
        let resp = get(&url, header_key, header_value);
        
        assert_eq!(resp.is_ok(), true)
    }
    
    #[test]
    fn test_post() {
        // API params
        let base_url = "https://dummyjson.com/products/";
        let header_key = "Content-Type"; 
        let header_value = "application/json";
        let product_endpoint = "add";
    
        let url = format!("{}{}", base_url, product_endpoint);
    
        // Post request
        let body:serde_json::Result<serde_json::Value> = serde_json::from_str("{\"title\": \"Iphone Galaxy + 1\"}");
        let resp = post(&url, header_key, header_value, body.expect("JSON").to_string());
    
        assert_eq!(resp.is_ok(), true)
    
    }
    
    #[test]
    fn test_put() {
        // API params
        let base_url = "https://dummyjson.com/products/";
        let header_key = "Content-Type"; 
        let header_value = "application/json";
        let product_endpoint = "1";
    
        let url = format!("{}{}", base_url, product_endpoint);
    
        // Put request
        let body:serde_json::Result<serde_json::Value>  = serde_json::from_str("{\"title\": \"iPhone Galaxy +1\"}");
        let resp = put(&url, header_key, header_value, body.expect("JSON").to_string());
    
        assert_eq!(resp.is_ok(), true)
    }
    
    #[test]
    fn test_delete() {
        // API params
        let base_url = "https://dummyjson.com/products/";
        let header_key = "Content-Type"; 
        let header_value = "application/json";
        let product_endpoint = "1";
    
        let url = format!("{}{}", base_url, product_endpoint);
    
        // Delete request
        let body:serde_json::Result<serde_json::Value>  = serde_json::from_str("{\"title\": \"iPhone Galaxy +1\"}");
        let resp = delete(&url, header_key, header_value, body.expect("JSON").to_string());
    
        assert_eq!(resp.is_ok(), true)
    
    }
}

