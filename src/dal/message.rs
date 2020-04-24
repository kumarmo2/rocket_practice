use crate::models::{Message};
use crate::dtos::{MessageCreateRequest};
use crate::schema::*;
use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::{insert_into};


pub fn create(dto: &MessageCreateRequest, conn: &MysqlConnection) -> bool {
    use crate::schema::messages::dsl::*;
    let result = insert_into(messages)
        .values(dto)
        .execute(conn);
    match result {
        Ok(_) => {
            true
        },
        Err(reason) => {
            println!("message insertion failed: {}", reason);
            false
        }
    }
}
