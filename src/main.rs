mod models;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use models::blockchain::Blockchain;
use serde::Deserialize;

// Initialize the Blockchain
fn init_blockchain(difficulty: usize) -> Blockchain {
    Blockchain::new(difficulty)
}

// Shared state for blockchain
struct AppState {
    blockchain: Mutex<Blockchain>
}

// Handler to add a new block
async fn add_block(data: web::Json<AddBlockData>, state: web::Data<AppState>) -> impl Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    blockchain.add_block(data.data.clone());

    HttpResponse::Ok().json(blockchain.chain.last())
}

// Handler to get the whole blockchain
async fn get_blockchain(state: web::Data<AppState>) -> impl Responder {
    let blockchain = state.blockchain.lock().unwrap();
    HttpResponse::Ok().json(&*blockchain.chain)
}

// Define the structure for the block data payload
#[derive(Deserialize)]
struct AddBlockData {
    data: String,
}

// Main function to start the HTTP server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let blockchain = init_blockchain(4);
    let shared_data = web::Data::new(AppState {
        blockchain: Mutex::new(blockchain),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .route("/blockchain", web::get().to(get_blockchain))
            .route("/blockchain/add", web::post().to(add_block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}