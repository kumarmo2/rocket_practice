use crate::business::room;
use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::{CreateRoomRequest, RoomDto};
use crate::models::{CounterWrapper, CustomKey, MySqlDb};

use rocket::http::{self, Status};
use rocket::State;
use rocket_contrib::json::Json;

use chat_common_types::dtos::RoomInfo;

use std::thread;

#[post("/", data = "<request>")]
pub fn create(api_key: ApiKey, request: Json<CreateRoomRequest>, conn: MySqlDb) -> http::Status {
    println!("create room request: {:?}", request);
    if let Some(reason) = validate_create_room_request(&request) {
        println!("bad request");
        return http::Status::new(400, reason);
    }
    println!("{:?}", request);
    match room::create(request.into_inner(), &conn) {
        Ok(_) => {
            return http::Status::new(201, "Created");
        }
        Err(reason) => {
            //TODO: send appropriate response status.
            return http::Status::new(400, reason);
        }
    }
}

//TODO: there should be check that whoever is present or the creator of the room can only
// add any other member here.
#[post("/<id>/members", data = "<members_json>")]
pub fn add_members(
    id: i32,
    _api_key: ApiKey,
    members_json: Json<Vec<i32>>,
    conn: MySqlDb,
) -> Json<Result<bool, &'static str>> {
    println!("members: {:?}", &members_json);
    match validate_add_members_request(&members_json) {
        Some(reason) => {
            println!("validation error: {}", reason);
            return Json(Err(reason));
        }
        None => {}
    }
    Json(room::add_members(id, &members_json, &conn))
    // "adding member"
}

fn validate_add_members_request(member_ids: &[i32]) -> Option<&'static str> {
    if member_ids.len() > 8 {
        return Some("maximum 8 members allowed");
    }
    match member_ids.iter().find(|&id| *id < 1) {
        Some(_) => {
            // TODO: return appropriate status code.
            return Some("member id cannot be less than 1");
        }
        _ => {}
    }
    None
}

fn validate_create_room_request(request: &CreateRoomRequest) -> Option<&'static str> {
    if request.creator_user_id < 1 {
        return Some("Invalid user id");
    }
    None
}

#[get("/<id>")]
// pub fn get<'a>(id: u32) -> RoomDto<'a> {
pub fn get(
    id: i32,
    custom_key: State<CustomKey>,
    counter_wrapper: State<CounterWrapper>,
    conn: MySqlDb,
) -> Json<Option<RoomDto>> {
    if id < 1 {
        return Json(None);
    }
    counter_wrapper.inner().increment();
    let curr_thread = thread::current();
    println!(
        "thread, id: {:?}, name: {}",
        curr_thread.id(),
        curr_thread.name().unwrap_or("No Name"),
    );
    match room::get_by_id(id, &conn) {
        Ok(room_model) => {
            return Json(Some(RoomDto::from_room_model(room_model)));
        }
        Err(_) => {
            println!("not found");
            return Json(None);
        }
    }
}

#[get("/")]
pub fn get_all(_api_key: ApiKey, conn: MySqlDb) -> Json<Vec<RoomDto>> {
    // let mut rooms = Vec::new();
    // rooms.push(RoomDto::dummy_room_dto());
    match room::get_all(&conn) {
        Ok(rooms) => {
            // let result = Vec::new();
            return Json(
                rooms
                    .into_iter()
                    .map(|r| RoomDto::from_room_model(r))
                    .collect(),
            );
        }
        Err(reason) => {
            println!("some error: {}", reason);
            return Json(vec![]);
        }
    }
    // Json(rooms)
}

#[get("/<id>/info")]
pub fn get_room_info(_api_key: ApiKey, conn: MySqlDb, id: i32) -> Result<Json<RoomInfo>, Status> {
    if id < 1 {
        return Err(Status::new(400, "Invalid room id"));
    }
    match room::get_room_info(id, &conn) {
        Some(room) => Ok(Json(room)),
        // TODO: better response and error handling needs to be taken care of.
        _ => Err(Status::new(500, "Unknown error")),
    }
}

#[get("/<id>/members")]
pub fn get_room_member_ids(
    _api_key: ApiKey,
    id: i32,
    conn: MySqlDb,
) -> Result<Json<Vec<i32>>, Status> {
    if id < 1 {
        return Err(Status::new(400, "Invalid room id"));
    }
    match room::get_room_members(id, &conn) {
        Some(members) => Ok(Json(
            members
                .iter()
                .map(|room_subs| room_subs.member_id)
                .collect(),
        )),
        _ => Err(Status::new(500, "Something went wrong")),
    }
}
