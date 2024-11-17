use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use config::connect_db;
use models::block::Block;
use models::blockchain::Blockchain;
use sea_orm::ActiveValue;
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder};
use serde::Deserialize;
use std::sync::Mutex;

mod config;
mod models;

use entity::model::ActiveModel as BlockActiveModel;
use entity::model::Entity as BlockEntity;

async fn insert_block(db: &DatabaseConnection, block: &Block) -> Result<(), sea_orm::DbErr> {
    let block_to_insert = BlockActiveModel {
        id: ActiveValue::NotSet,
        index: ActiveValue::Set(block.index as i64),
        timestamp: ActiveValue::Set(block.timestamp as i64),
        proof_of_work: ActiveValue::Set(block.proof_of_work as i64),
        previous_hash: ActiveValue::Set(block.previous_hash.clone()),
        hash: ActiveValue::Set(block.hash.clone()),
        data: ActiveValue::Set(block.data.clone()),
    };

    BlockEntity::insert(block_to_insert)
        .exec(db)
        .await
        .map(|_| ())
}

// Initialize the Blockchain
async fn init_blockchain(difficulty: usize, db: &DatabaseConnection) -> Blockchain {
    // Fetch block from the database
    let blocks = BlockEntity::find()
        .order_by_asc(entity::model::Column::Index)
        .all(db)
        .await
        .unwrap_or_default();

    if !blocks.is_empty() {
        // Reconstruct the blockchain from fetched blocks
        let chain: Vec<Block> = blocks
            .into_iter()
            .map(|b| Block {
                index: b.index as u64,
                timestamp: b.timestamp as u64,
                proof_of_work: b.proof_of_work as u64,
                previous_hash: b.previous_hash,
                hash: b.hash,
                data: b.data,
            })
            .collect();

        let blockchain = Blockchain { chain, difficulty };

        // Validate the retrieved blockchain
        if blockchain.is_chain_valid() {
            println!("Loaded valid blockchain from database.");
            return blockchain;
        } else {
            println!(
                "Invalid blockchain in database. Clearing database and starting new blockchain"
            );

            // Delete all blocks from the database
            match BlockEntity::delete_many().exec(db).await {
                Ok(_) => println!("Invalid blocks deleted from database."),
                Err(err) => eprintln!("Failed to delete invalid blocks: {}", err),
            }
        }
    } else {
        // If no blocks in database, start a new blockchain
        println!("No blockchain found in database. Starting new blockchain.");
    };

    let new_blockchain = Blockchain::new(difficulty);

    // Store the genesis block in the database if the blockchain is new
    if let Some(genesis_block) = new_blockchain.chain.first() {
        if let Err(err) = insert_block(db, genesis_block).await {
            eprintln!("Failed to insert genesis block: {}", err);
        } else {
            println!("Genesis block inserted into the database.");
        }
    }
    new_blockchain
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
    let db: DatabaseConnection = connect_db().await;

    let blockchain = init_blockchain(4, &db).await;

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
