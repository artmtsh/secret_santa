pub mod json_structs;
pub mod models;
pub mod schema;
pub mod service;

use service::database_connection::Database;


use std::sync::{Arc, Mutex};

fn main() -> Result<(), std::io::Error> {
    let version: &'static str = env!("CARGO_PKG_VERSION");

    let f = async {
        let database = Database {};
        let state = Arc::new(Mutex::new(database));
        let mut app = tide::with_state(state);

        app.at("/version")
            .get(move |_| async move { Ok(format!("version: {version}")) });
        app.listen("127.0.0.1:8080").await
    };
    futures::executor::block_on(f)
}