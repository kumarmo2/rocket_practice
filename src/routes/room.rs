use crate::dtos::request_guards::ApiKey::ApiKey;
use crate::dtos::{CreateRoomRequest, RoomDto};
use crate::models::{CounterWrapper, CustomKey, MySqlDb, Room};
use rocket::State;
use rocket_contrib::json::Json;
use std::thread;

#[post("/", data = "<request>")]
pub fn create(request: Json<CreateRoomRequest>) -> String {
    println!("create room request: {:?}", request);
    String::from("create room request")
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
