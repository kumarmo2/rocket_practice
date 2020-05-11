use crate::dal::last_insert_id;
use crate::dtos::CreateRoomRequest;
use crate::models::Room;
use crate::utils;
use diesel::dsl::select;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::{result::Error, sql_types::Integer};

pub fn get_all(conn: &diesel::MysqlConnection) -> Result<Vec<Room>, &'static str> {
    use crate::schema::rooms::dsl::*;
    let results = rooms.load::<Room>(conn);
    match results {
        Ok(list) => Ok(list),
        Err(reason) => {
            println!("failed: {}", reason);
            return Err("some problems");
        }
    }
}

pub fn create_from_request(
    request: CreateRoomRequest,
    conn: &MysqlConnection,
) -> Result<i32, &'static str> {
    use crate::schema::rooms::dsl::*;
    let path = utils::generate_v4_base64_uuid();
    let room_name = request.room_name.unwrap_or(String::new());
    let is_public_room = request.is_public.unwrap_or(false);
    match diesel::insert_into(rooms)
        .values((
            creator_user_id.eq(request.creator_user_id),
            url_identifier.eq(path),
            is_public.eq(is_public_room),
            name.eq(room_name),
        ))
        .execute(conn)
    {
        Ok(_) => {
            // TODO: need to see how can I return the id in the same query.
            let ids = select(last_insert_id).load(conn).unwrap();
            return Ok(ids[0]);
        }
        Err(reason) => {
            println!("error creating room: {}", reason);
            return Err("could not create room");
        }
    }
}

pub fn get_by_id(id: i32, conn: &MysqlConnection) -> Result<Room, &'static str> {
    let query = "select * from rooms where id = ? limit 1";
    let result = diesel::sql_query(query)
        .bind::<Integer, _>(id)
        .get_result::<Room>(conn);
    match result {
        Ok(r) => {
            return Ok(r);
        }
        Err(reason) => {
            println!("reason: {}", reason);
            return Err("some error");
        }
    }
}

pub fn get_rooms_from_ids(ids: &[i32], conn: &MysqlConnection) -> Result<Vec<Room>, Error> {
    use crate::schema::rooms::dsl::*;
    rooms.filter(id.eq_any(ids)).load(conn)
}
