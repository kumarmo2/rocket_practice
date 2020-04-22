use crate::business::room;
use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::{CreateRoomRequest, RoomDto};
use crate::models::{CounterWrapper, CustomKey, MySqlDb, Room};
use rocket::http;
use rocket::State;
use rocket_contrib::json::Json;
use std::thread;

#[post("/", data = "<request>")]
// pubfn create(api_key: ApiKey, request: Json<CreateRoomRequest>) -> http::Status {
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

fn validate_create_room_request(request: &CreateRoomRequest) -> Option<&'static str> {
    if request.creator_user_id < 1 {
        return Some("Invalid user id");
    }
    None
}

#[get("/<id>")]
// pub fn get<'a>(id: u32) -> RoomDto<'a> {
pub fn get(
    id: u32,
    custom_key: State<CustomKey>,
    counter_wrapper: State<CounterWrapper>,
    conn: MySqlDb,
) -> RoomDto {
    let room = Room::get_dummy();
    let result = RoomDto::from_room_model(room);
    counter_wrapper.inner().increment();
    let curr_thread = thread::current();
    println!(
        "thread, id: {:?}, name: {}",
        curr_thread.id(),
        curr_thread.name().unwrap_or("No Name"),
    );
    result
}

#[get("/")]
pub fn get_all(api_key: ApiKey, conn: MySqlDb) -> Json<Vec<RoomDto>> {
    // let mut rooms = Vec::new();
    // rooms.push(RoomDto::dummy_room_dto());
    match room::get_all(&conn) {
        Ok(rooms) => {
            // let result = Vec::new();
               return Json(rooms.into_iter()
                .map(|r| RoomDto::from_room_model(r))
                .collect());
            
        },
        Err(reason)  => {
            println!("some error: {}", reason);
            return Json(vec![]);
        }
    }
    // Json(rooms)
}
