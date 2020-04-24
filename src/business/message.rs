use crate::dtos::{MessageCreateRequest};
use crate::dal::{room, user, message, room_subscribers};
use diesel::MysqlConnection;
use diesel::result::Error;


pub fn create(messageCreateRequest: &MessageCreateRequest, conn: &MysqlConnection) -> Result<bool, Error>{
    match room_subscribers::get(messageCreateRequest.room_id, messageCreateRequest.sender_id, conn) {
        Ok(sub) => {
            println!("sub found: {:?}", sub);
        },
        Err(error) => {
            return Err(error);
        }
    }
    Ok(message::create(messageCreateRequest, conn))

    // match room::get_by_id(messageCreateRequest.room_id, conn) {
    //     Ok(_) => {},
    //     Err(reason) => {
    //         return false;
    //     }
    // }

    // match user::get_by_id(messageCreateRequest.sender_id, conn) {
    //     Ok(_) => {},
    //     Err(reason) => {
    //         return false;
    //     }
    // }

}