use crate::business::user;
use crate::dtos::CreateRoomRequest;
use crate::models::{Room, User};
use diesel::MysqlConnection;

pub fn create(
    create_request: CreateRoomRequest,
    conn: &MysqlConnection,
) -> Result<(), &'static str> {
    match User::get_by_id(create_request.creator_user_id, conn) {
        Ok(_) => {}
        Err(reason) => {
            return Err(reason);
        }
    }
    return Room::create_from_request(create_request, conn);
}

pub fn get_all(conn: &MysqlConnection) -> Result<Vec<Room>, &'static str>{
    Room::get_all(conn)
}
