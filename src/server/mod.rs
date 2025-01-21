use anyhow::Result;
use pallas::ledger::traverse::MultiEraTx;
use rocket::{data::ToByteUnit, http::Status, post, routes, Data, State};
use crate::storage::in_memory_db::CborTransactionsDb;

pub mod utxorpc;


#[post("/api/submit/tx", format = "application/cbor", data = "<cbor_data>")]
async fn submit_tx(
    cbor_data: Data<'_>,
    cbor_txs_db: &State<CborTransactionsDb>,
) -> Result<String, Status> {
    // Limit how many bytes we read (16 KB here).
    let max_size = 16.kilobytes();

    // Read the raw bytes from the request body.
    let bytes = match cbor_data.open(max_size).into_bytes().await {
        Ok(buf) => buf,
        Err(_) => return Err(Status::PayloadTooLarge),
    };

    // The `bytes.value` is a `Vec<u8>` containing the raw CBOR data.
    let raw_cbor = bytes.value;

    tracing::info!("Tx Cbor: {:?}", hex::encode(&raw_cbor));

    let parsed_tx = MultiEraTx::decode(&raw_cbor).unwrap();
    let tx_hash = parsed_tx.hash();

    // Store the transaction in our mempool.
    // We'll lock the mutex, then push the new transaction.
    cbor_txs_db.enqueue_tx(raw_cbor.clone());
    // Return the transaction hash as a response.
    Ok(hex::encode(tx_hash))
}


pub async fn run(cbor_txs_db: CborTransactionsDb) -> Result<()> {
    let _ = rocket::build()
        .manage(cbor_txs_db)
        .mount("/", routes![submit_tx])
        .launch()
        .await?;
    Ok(())
}
