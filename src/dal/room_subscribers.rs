use crate::models::RoomSubscriber;
use crate::dtos::RoomSubscriberInsertableDto;
use diesel::mysql::MysqlConnection;
use crate::schema::*;
use diesel::{insert_into, result:: Error};
use diesel::prelude::*;
use diesel::sql_types::Integer;



pub fn create(room_subscriber: &RoomSubscriberInsertableDto, conn: &MysqlConnection) -> Result<(), &'static str> {
    use crate::schema::roomsubscribers::dsl::*;

    // let result = insert_into(roomsubscribers::table)
    let result = insert_into(roomsubscribers)
        .values(room_subscriber)
        .execute(conn);

    match result {
        Ok(_) => {
            return Ok(());
        },
        Err(reason) => {
            println!("error inserting roomsubscriptions: {}",reason );
            return Err("some error");
        }
    }
}

pub fn add_members_to_room(subscribers: &[RoomSubscriberInsertableDto], conn: &MysqlConnection) -> Result<(), &'static str> {
    use crate::schema::roomsubscribers::dsl::*;
    let result = insert_into(roomsubscribers)
                    .values(subscribers)
                    .execute(conn);

    match result {
        Ok(_) => {
            return Ok(());
        },
        Err(reason) => {
            println!("error inserting roomsubscriptions: {}",reason );
            return Err("some error");
        }
    }
}

pub fn get(room_id_input: i32, sender_id: i32, conn: &MysqlConnection) -> Result<RoomSubscriber, Error> {
    use crate::schema::roomsubscribers::dsl::*;

    let query = roomsubscribers
                .filter(member_id.eq(sender_id))
                .filter(room_id.eq(room_id_input));
        
    // println!("query: {}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query));

             query.first::<RoomSubscriber>(conn)

    // match results {
    //     Ok(list) => {
    //     },
    //     Err(reason) => {
    //         return Err(reason);
    //     }
    // }

    // let query = "select * from roomsubscribers where room_id = ? and sender_id = ? limit 1";
    // let results = diesel::sql_query(query)
    //                 .bind::<Integer, _>((room_id,sender_id))
    //                 // .bind::<Integer, _>(room_id)
    //                 .get_result::<RoomSubscriber>(conn);
            

}