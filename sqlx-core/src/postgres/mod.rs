//! **Postgres** database and connection types.

pub use arguments::PgArguments;
pub use connection::PgConnection;
pub use cursor::PgCursor;
pub use database::Postgres;
pub use error::PgDatabaseError;
pub use listen::{PgListener, PgNotification};
pub use row::PgRow;
pub use types::PgTypeInfo;
pub use value::{PgData, PgValue};

mod arguments;
mod connection;
mod cursor;
mod database;
mod error;
mod executor;
mod listen;
mod protocol;
mod row;
mod sasl;
mod stream;
mod tls;
pub mod types;
mod value;

/// An alias for [`Pool`][crate::pool::Pool], specialized for **Postgres**.
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub type PgPool = crate::pool::Pool<PgConnection>;

make_query_as!(PgQueryAs, Postgres, PgRow);
impl_map_row_for_row!(Postgres, PgRow);
impl_from_row_for_tuples!(Postgres, PgRow);
