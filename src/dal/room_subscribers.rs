use crate::models::RoomSubscriber;
use crate::dtos::RoomSubscriberInsertableDto;
use diesel::mysql::MysqlConnection;
use crate::schema::*;
use diesel::{insert_into};
use diesel::prelude::*;



pub fn create(room_subscriber: &RoomSubscriberInsertableDto, conn: &MysqlConnection) -> Result<(), &'static str> {
    use crate::schema::roomsubscribers::dsl::*;

    // let result = insert_into(roomsubscribers::table)
    let result = insert_into(roomsubscribers)
        .values(room_subscriber)
        .execute(conn);

    match result {
        Ok(_) => {
            return Ok(());
        },
        Err(reason) => {
            println!("error inserting roomsubscriptions: {}",reason );
            return Err("some error");
        }
    }
}

pub fn add_members_to_room(subscribers: &[RoomSubscriberInsertableDto], conn: &MysqlConnection) -> Result<(), &'static str> {
    use crate::schema::roomsubscribers::dsl::*;
    let result = insert_into(roomsubscribers)
                    .values(subscribers)
                    .execute(conn);

    match result {
        Ok(_) => {
            return Ok(());
        },
        Err(reason) => {
            println!("error inserting roomsubscriptions: {}",reason );
            return Err("some error");
        }
    }
}