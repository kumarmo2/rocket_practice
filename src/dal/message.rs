use crate::dal::last_insert_id;
use crate::dtos::MessageCreateRequest;
use crate::models::Message;
use crate::schema::*;
use diesel::dsl::select;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};
use diesel::MysqlConnection;

// TODO: remove dependency of diesel.
pub fn create(dto: &MessageCreateRequest, conn: &MysqlConnection) -> Option<i32> {
    use crate::schema::messages::dsl::*;

    let result = insert_into(messages).values(dto).execute(conn);

    match result {
        Ok(_) => {
            // TODO: reduce these 2 db calls to a single one.
            let ids = select(last_insert_id).load(conn).unwrap();
            return Some(ids[0]);
        }
        Err(reason) => {
            println!("message insertion failed: {}", reason);
            return None;
        }
    }
}
