pub mod mempool;
pub mod stage;
pub mod tx_submit_peer;
pub mod tx_submit_peer_manager;

pub use stage::Stage;

// Test for Fanout Stage
#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::in_memory_db::CborTransactionsDb;
    use crate::{Config, PeerManagerConfig};
    use std::net::SocketAddr;
    use std::str::FromStr;
    use std::sync::{Arc, Mutex};
    use std::vec;

    #[test]
    fn test_fanout_stage() {
        let cbor_txs_db = CborTransactionsDb {
            cbor_txs: Arc::new(Mutex::new(vec![vec![1, 2, 3]])),
        };

        let config = Config {
            peer_manager: PeerManagerConfig {
                peers: vec!["".to_string()],
            },
            server: crate::server::Config {
                listen_address: SocketAddr::from_str("[::1]:50052").unwrap(),
            },
            storage: crate::storage::Config {
                db_path: "dev.db".into(),
            },
        };

        // Run mock node

        // Run Fanout Stage
        let fanout = Stage::new(cbor_txs_db, config);
    }
}
