use std::str::FromStr;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
};

// Request and Response Structs
#[derive(Deserialize)]
struct TransferRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
    data: InstructionResponse,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Serialize)]
struct InstructionResponse {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

// Endpoint Handlers
#[get("/keypair")]
async fn generate_keypair() -> impl Responder {
    let wallet = Keypair::new();
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "pubkey": wallet.pubkey().to_string(), 
            "secret": bs58::encode(wallet.to_bytes()).into_string() 
        }
    }))
}

#[post("/send/sol")]
async fn send_sol(req: web::Json<TransferRequest>) -> HttpResponse {
    // Validate from pubkey
    let from_pubkey = match Pubkey::from_str(&req.from) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid from pubkey format".to_string(),
            });
        }
    };

    // Validate to pubkey
    let to_pubkey = match Pubkey::from_str(&req.to) {
        Ok(pubkey) => pubkey,
        Err(_) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                success: false,
                error: "Invalid to pubkey format".to_string(),
            });
        }
    };

    // Validate lamports amount
    if req.lamports == 0 {
        return HttpResponse::BadRequest().json(ErrorResponse {
            success: false,
            error: "Lamports amount must be greater than 0".to_string(),
        });
    }

    // Create SOL transfer instruction
    let instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, req.lamports);

    // Prepare response
    let response = SuccessResponse {
        success: true,
        data: InstructionResponse {
            program_id: solana_sdk::system_program::id().to_string(),
            accounts: instruction.accounts.iter().map(|acc| acc.pubkey.to_string()).collect(),
            instruction_data: bs58::encode(&instruction.data).into_string(),
        },
    };

    HttpResponse::Ok().json(response)
}

// New hello route
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(generate_keypair)
            .service(send_sol)
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}