use crate::dtos::RoomSubscriberInsertableDto;
use crate::models::RoomSubscriber;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::{insert_into, result::Error};

pub fn add_members_to_room(
    subscribers: &[RoomSubscriberInsertableDto],
    conn: &MysqlConnection,
) -> Result<(), &'static str> {
    use crate::schema::roomsubscribers::dsl::*;
    let result = insert_into(roomsubscribers)
        .values(subscribers)
        .execute(conn);

    match result {
        Ok(_) => {
            return Ok(());
        }
        Err(reason) => {
            println!("error inserting roomsubscriptions: {}", reason);
            return Err("some error");
        }
    }
}

pub fn get(
    room_id_input: i32,
    sender_id: i32,
    conn: &MysqlConnection,
) -> Result<RoomSubscriber, Error> {
    use crate::schema::roomsubscribers::dsl::*;

    let query = roomsubscribers
        .filter(member_id.eq(sender_id))
        .filter(room_id.eq(room_id_input));

    query.first::<RoomSubscriber>(conn)
}

pub fn get_members(room_id_input: i32, conn: &MysqlConnection) -> Option<Vec<RoomSubscriber>> {
    use crate::schema::roomsubscribers::dsl::*;
    match roomsubscribers.filter(room_id.eq(room_id_input)).load(conn) {
        Ok(list) => Some(list),
        Err(reason) => {
            println!("could not fetch room subscribers: {}", reason);
            None
        }
    }
}

pub fn get_rooms_of_user(
    user_id_input: i32,
    conn: &MysqlConnection,
) -> Result<Vec<RoomSubscriber>, Error> {
    use crate::schema::roomsubscribers::dsl::*;

    roomsubscribers
        .filter(member_id.eq(user_id_input))
        .load(conn)
}
