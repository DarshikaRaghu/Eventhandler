#![allow(non_snake_case)]
use reqwest;
use std::error::Error;
use serde_json::Value;
use serde::Deserialize;

// Define a struct to represent the response structure
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ContractResponse {
    status: String,
    message: String,
    result: String, // Change the type to String
}

struct EtherscanIntegration {
    api_key: String,
    fetch_abi_endpoint: String,
}


impl EtherscanIntegration {
    fn new(api_key: &str, fetch_abi_endpoint: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            fetch_abi_endpoint: fetch_abi_endpoint.to_string(),
        }
    }

    fn create_api_url(&self, contract_address: &str) -> String {
        format!(
            "{}",
            self.fetch_abi_endpoint
                .replace(":apiKey", &self.api_key)
                .replace(":address", contract_address)
        )
    }
}

#[tokio::main]
async fn fun() -> Result<(), Box<dyn Error>> {
    // Example usage
    let etherscan_integration =
        EtherscanIntegration::new("I1SVQDDQV9QM4TZBU3B3YSFFPM4INJXHEM", "https://api.etherscan.io/api?module=contract&action=getabi&address=:address&apikey=:apiKey");
    let contract_address = "0x0581ddf7a136c6837429a46c6cb7b388a3e52971";
    let api_url = etherscan_integration.create_api_url(contract_address);

    let response = reqwest::get(&api_url).await?;

     if response.status().is_success() {
                  // Get the response body as a string
        let response_body = response.text().await?;

        // Parse the response body into the ContractResponse struct
        let contract_response: ContractResponse = serde_json::from_str(&response_body)?;

        // Deserialize the inner JSON string
        let result_json: Value = serde_json::from_str(&contract_response.result)?;

        // Access fields of the inner JSON
        println!("Result JSON: {}", result_json);
        Ok(())
    } else {
         Err(format!("Non-success status code: {}", response.status()).into())
    }
   
}
