use crate::dal::{message, room, room_subscribers, user};
use crate::dtos::MessageCreateRequest;
use chat_common_types::events::{MessageEvent, MessageEventType};
use diesel::result::Error;
use diesel::MysqlConnection;
use manager::RabbitMqManager;

pub fn create(
    messageCreateRequest: &MessageCreateRequest,
    conn: &MysqlConnection,
    rabbit: &RabbitMqManager,
) -> Option<i32> {
    match room_subscribers::get(
        messageCreateRequest.room_id,
        messageCreateRequest.sender_id,
        conn,
    ) {
        Ok(sub) => {
            println!("sub found: {:?}", sub);
        }
        Err(error) => {
            println!("errro: {}", error);
            return None;
        }
    }
    match message::create(messageCreateRequest, conn) {
        Some(id) => {
            send_message_event(
                id,
                MessageEventType::Send,
                messageCreateRequest.sender_id,
                rabbit,
            );
            return Some(id);
        }
        None => {
            println!("could not create message");
            return None;
        }
    }
}

fn send_message_event(
    id: i32,
    message_type: MessageEventType,
    user_id: i32,
    rabbit: &RabbitMqManager,
) {
    let event = MessageEvent {
        id,
        user_id,
        event_type: MessageEventType::Send,
    };

    rabbit.publish_message_to_queue_sync("messages", &event);
}
