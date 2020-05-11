use crate::dal::room_subscribers;
use crate::dal::{queue, room, user as user_dal};
use crate::dtos::{CreateRoomRequest, RoomSubscriberInsertableDto};
use crate::models::{self, Room, RoomSubscriber};
use chat_common_types::dtos::RoomInfo;
use diesel::MysqlConnection;

pub fn create(
    create_request: CreateRoomRequest,
    conn: &MysqlConnection,
) -> Result<i32, &'static str> {
    match user_dal::get_by_id(create_request.creator_user_id, conn) {
        Ok(_) => {}
        Err(reason) => {
            return Err(reason);
        }
    }

    // Had to save the user id in local variable as I didn't design the dal layer correctly,
    // its taking the ownership of the request and we need request.id later on.
    // Since correcting the dal api will take some time, I am saving id in local variable for now.
    let user_id = create_request.creator_user_id;
    let room_id;
    match room::create_from_request(create_request, conn) {
        Ok(id) => {
            room_id = id;
        }
        Err(reason) => {
            return Err(reason);
        }
    }
    let mut subs = vec![];
    subs.push(RoomSubscriberInsertableDto::new(user_id, room_id));
    match room_subscribers::add_members_to_room(&subs, conn) {
        Ok(_) => Ok(room_id),
        Err(reason) => Err(reason),
    }
}

pub fn get_by_id(id: i32, conn: &MysqlConnection) -> Result<Room, &'static str> {
    room::get_by_id(id, conn)
}

pub fn get_all(conn: &MysqlConnection) -> Result<Vec<Room>, &'static str> {
    room::get_all(conn)
}

pub fn add_members(
    id: i32,
    member_ids: &[i32],
    conn: &MysqlConnection,
) -> Result<bool, &'static str> {
    match room::get_by_id(id, conn) {
        Ok(_) => {}
        Err(reason) => {
            println!("could not find the room: {}", reason);
            return Err("could not find the room");
        }
    }
    let subs: Vec<_> = member_ids
        .iter()
        .map(|&member_id| RoomSubscriberInsertableDto::new(member_id, id))
        .collect();
    match room_subscribers::add_members_to_room(&subs, conn) {
        Ok(_) => {
            return Ok(true);
        }
        Err(reason) => {
            println!("reason for failure: {}", reason);
            return Ok(false);
        }
    }
}

pub fn get_room_members(id: i32, conn: &MysqlConnection) -> Option<Vec<RoomSubscriber>> {
    room_subscribers::get_members(id, conn)
}

pub fn get_room_info(id: i32, conn: &MysqlConnection) -> Option<RoomInfo> {
    let room: Room;
    match get_by_id(id, conn) {
        Ok(room_model) => {
            room = room_model;
        }
        Err(reason) => {
            println!("no room fetched: {}", reason);
            return None;
        }
    }
    let subs: Vec<_>;
    match get_room_members(id, conn) {
        Some(list) => {
            subs = list;
        }
        _ => {
            return None;
        }
    }
    let member_ids: Vec<i32> = subs
        .iter()
        .map(|member: &RoomSubscriber| member.member_id)
        .collect();

    Some(RoomInfo {
        id: room.id,
        name: room.name,
        path: room.url_identifier,
        member_ids,
    })
}

pub fn get_room_members_client_queues(
    id: i32,
    conn: &MysqlConnection,
) -> Option<Vec<models::Queue>> {
    let members: Vec<models::RoomSubscriber>;
    match get_room_members(id, conn) {
        Some(m) => {
            members = m;
        }
        None => {
            return None;
        }
    }
    let member_ids: Vec<i32> = members.iter().map(|member| member.member_id).collect();
    queue::get_queues_from_user_ids(member_ids.as_ref(), conn)
}
