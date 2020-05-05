pub mod message;
pub mod queue;
pub mod room;
pub mod room_subscribers;
pub mod user;

use diesel::sql_types::Integer;

no_arg_sql_function!(last_insert_id, Integer);
