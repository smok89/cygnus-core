use std::sync::Mutex;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use config::connect_db;
use models::blockchain::Blockchain;
use serde::Deserialize;
use sea_orm::{DatabaseConnection, EntityTrait};
use sea_orm::ActiveValue;

mod models;
mod config;

use entity::model::Entity as BlockEntity;
use entity::model::ActiveModel as BlockActiveModel;

// Initialize the Blockchain
fn init_blockchain(difficulty: usize) -> Blockchain {
    Blockchain::new(difficulty)
}

// Shared state for blockchain
struct AppState {
    blockchain: Mutex<Blockchain>,
    db: DatabaseConnection,
}

// Handler to add a new block
async fn add_block(data: web::Json<AddBlockData>, state: web::Data<AppState>) -> impl Responder {
    let mut blockchain = state.blockchain.lock().unwrap();
    blockchain.add_block(data.data.clone());
    let last_block = blockchain.chain.last().unwrap().clone();

    // Prepare block for database insertion
    let block_to_insert = BlockActiveModel {
        id: ActiveValue::NotSet,
        index: ActiveValue::Set(last_block.index as i64),
        timestamp: ActiveValue::Set(last_block.timestamp as i64),
        proof_of_work: ActiveValue::Set(last_block.proof_of_work as i64),
        previous_hash: ActiveValue::Set(last_block.previous_hash.clone()),
        hash: ActiveValue::Set(last_block.hash.clone()),
        data: ActiveValue::Set(last_block.data.clone()),
    };

    // Insert into the database
    match BlockEntity::insert(block_to_insert).exec(&state.db).await {
        Ok(_) => HttpResponse::Ok().json(last_block),
        Err(err) => {
            eprintln!("Failed to insert block: {}", err);
            HttpResponse::InternalServerError().body("Failed to persist block")
        }
    }
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

    let db: DatabaseConnection = config::connect_db().await;

    let shared_data = web::Data::new(AppState {
        blockchain: Mutex::new(blockchain), 
        db,
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