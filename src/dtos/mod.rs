pub mod request_guards;

use crate::models::Room;
use crate::schema::users;
use crate::schema::*;
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use diesel::{Insertable};

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     pub id: u32,
//     pub name: String,
//     pub age: u32,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    #[serde(rename = "creatorUserId")]
    pub creator_user_id: i32,
    #[serde(rename = "roomName")]
    pub room_name: Option<String>,
    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,
    // pub members: Option<Vec<u32>>,
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
// #[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub age: i32,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
// pub struct RoomDto<'a> {
pub struct RoomDto {
    pub id: i32,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMembersToRoomRequest {
    #[serde(rename = "memberIds")]
    member_ids: Vec<i32>,
}


#[derive(Insertable)]
#[table_name = "roomsubscribers"]
pub struct RoomSubscriberInsertableDto {
    pub member_id: i32,
    pub room_id: i32,
}

impl RoomSubscriberInsertableDto {
    pub fn new(member_id: i32, room_id: i32) -> Self {
        RoomSubscriberInsertableDto {
            member_id,
            room_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInsertableDto {
    pub room_id: i32,
    pub sender_id: i32,
    pub content: String,
}

impl MessageInsertableDto {
    pub fn new(room_id: i32, sender_id: i32, content: String) -> Self {
        Self {
            room_id,
            sender_id,
            content,
        }
    }
}

// impl<'a> RoomDto<'a> {
impl RoomDto {
    // pub fn from_room_model(model: &Room) -> RoomDto {
    pub fn from_room_model(model: Room) -> RoomDto {
        RoomDto {
            id: model.id,
            // name: &model.name,
            name: model.name,
            // path: &model.path,
            path: model.url_identifier,
        }
    }

    pub fn dummy_room_dto() -> RoomDto {
        RoomDto {
            id: 45,
            name: "some_room".to_string(),
            path: "dsfgdg".to_string(),
        }
    }
}

// impl<'r> Responder<'r> for RoomDto<'_> {
impl<'r> Responder<'r> for RoomDto {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(format!(
                "Room's name: {}, id: {}, path: {}",
                self.name, self.id, self.path
            )))
            .ok()
    }
}
