use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub(crate) enum PgConn {
    Pool(PgPool),
    Tx(Arc<Mutex<Transaction<'static, Postgres>>>),
}