use crate::dal::{queue, user};
use crate::models::{Room, RoomSubscriber, User};
use crate::utils::generate_v4_base64_uuid;

use crate::dal::{room, room_subscribers::get_rooms_of_user};
use diesel::{result::Error, MysqlConnection};

use manager::{AMQPValue, FieldTable, LongInt, QueueDeclareOptions, RabbitMqManager, ShortString};
use smol::block_on;
use std::collections::BTreeMap;

pub fn get_user_by_id(id: i32, conn: &MysqlConnection) -> Result<User, &'static str> {
    user::get_by_id(id, conn)
}

pub fn get_user_by_email(email: &str, conn: &MysqlConnection) -> Result<User, Error> {
    user::get_by_email(email, conn)
}

pub fn get_rooms(id: i32, conn: &MysqlConnection) -> Result<Vec<Room>, Error> {
    let room_subscriptions: Vec<RoomSubscriber>;
    match get_rooms_of_user(id, conn) {
        Ok(res) => {
            room_subscriptions = res;
        }
        Err(reason) => {
            println!("error fetching room of user, reasons: {:?}", reason);
            return Err(reason);
        }
    }

    let room_ids: Vec<i32> = room_subscriptions.iter().map(|sub| sub.room_id).collect();
    room::get_rooms_from_ids(&room_ids, conn)
}

pub fn register_user(id: i32, rabbit: &RabbitMqManager, conn: &MysqlConnection) -> Option<String> {
    let queue_name = generate_v4_base64_uuid();
    let queue_declare_options = QueueDeclareOptions {
        durable: false,
        exclusive: false,
        auto_delete: false,
        ..QueueDeclareOptions::default()
    };

    // TODO: since most of the arguements for queue_declare is always same.
    // can we make it const/static.
    let queue_arguements = BTreeMap::new();
    let mut queue_args_field_table = FieldTable::from(queue_arguements);

    // set max length of the queue to be 500. after that messages from the head will be truncated
    // that is the default behavior of RabbitMq.
    queue_args_field_table.insert(
        ShortString::from("x-max-length".to_string()),
        AMQPValue::LongInt(500 as LongInt),
    );

    // set max expiry time. if no consumer consumes from the queue, queue shall be deleted.
    queue_args_field_table.insert(
        ShortString::from("x-expires"),
        AMQPValue::LongInt(300000 as LongInt),
    );

    match block_on(rabbit.get_channel_pool().get().unwrap().queue_declare(
        &queue_name,
        queue_declare_options,
        queue_args_field_table,
    )) {
        Ok(_) => {}
        Err(reason) => {
            println!("error in declaring queue: {}", reason);
            return None;
        }
    }

    match queue::create(id, &queue_name, conn) {
        Some(_) => Some(queue_name),
        _ => None,
    }
}
