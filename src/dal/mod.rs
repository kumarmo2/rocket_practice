pub mod message;
pub mod room;
pub mod room_subscribers;
pub mod user;

use diesel::sql_types::{Integer, Text};

no_arg_sql_function!(last_insert_id, Integer);
