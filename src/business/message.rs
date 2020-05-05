use crate::dal::{message, room_subscribers};
use crate::dtos::MessageCreateRequest;
use chat_common_types::events::{MessageEvent, MessageEventType};
use diesel::MysqlConnection;
use manager::RabbitMqManager;

pub fn create(
    message_create_request: &MessageCreateRequest,
    conn: &MysqlConnection,
    rabbit: &RabbitMqManager,
) -> Option<i32> {
    match room_subscribers::get(
        message_create_request.room_id,
        message_create_request.sender_id,
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
    match message::create(message_create_request, conn) {
        Some(id) => {
            send_message_event(
                id,
                MessageEventType::Send,
                message_create_request.sender_id,
                message_create_request.room_id,
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
    message_event_type: MessageEventType,
    user_id: i32,
    room_id: i32,
    rabbit: &RabbitMqManager,
) {
    let event = MessageEvent {
        id,
        user_id,
        event_type: message_event_type,
        room_id,
    };

    rabbit.publish_message_to_queue_sync("messages", &event);
}
