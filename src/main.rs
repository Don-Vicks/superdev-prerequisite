use std::str::FromStr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use solana_sdk::signature::{Keypair, Signer};
use solana_client::nonblocking::rpc_client::RpcClient;  // Changed to nonblocking
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{system_instruction::transfer};
use serde::Deserialize;


// Structs used for Requests
#[derive(Deserialize)]
struct TransferRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[get("/keypair")]
async fn generate_keypair() -> impl Responder {
    let wallet = Keypair::new();
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "pubkey": wallet.pubkey().to_string(), 
            "secret": bs58::encode(wallet.secret_bytes()).into_string() 
        }
    }))
}

#[post("/send/sol")]
async fn send_sol(req: web::Json<TransferRequest>) -> HttpResponse {

    // Validate from pubkey
    if let Err(_) = Pubkey::from_str(&req.from) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Invalid from pubkey format"
        }));
    }

    // Validate to pubkey
    if let Err(_) = Pubkey::from_str(&req.to) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Invalid to pubkey format"
        }));
    }

    // Validate lamports amount
    if req.lamports == 0 {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "error": "Lamports amount must be greater than 0"
        }));
    }

    // If we get here, all validations passed
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "from": req.from,
        "to": req.to,
        "lamports": req.lamports
    }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(generate_keypair)
            .service(send_sol)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}