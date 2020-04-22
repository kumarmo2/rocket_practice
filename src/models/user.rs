// use super::User;
// use crate::dtos::CreateUserRequest;
// use crate::schema::*;
// use diesel::mysql::MysqlConnection;
// use diesel::prelude::*;
// use diesel::result::Error;

// impl User {
//     pub fn get_by_id(id: u32, conn: &MysqlConnection) -> Result<User, &'static str> {
//         use crate::schema::users::dsl::*;
//         // let results: Vec<User> = users.filter(id.eq(id)).load::<User>(conn);
//         let results = users.filter(id.eq(id)).load::<User>(conn);
//         match results {
//             Ok(mut list) => {
//                 if list.len() < 1 {
//                     return Err("No user found");
//                 } else {
//                     return Ok(list.remove(0));
//                 }
//             }
//             Err(reason) => {
//                 // Log the error
//                 return Err("Error while fetching results");
//             }
//         }
//     }

//     pub fn create_from_request(
//         create_request: &CreateUserRequest,
//         conn: &MysqlConnection,
//     ) -> Result<(), &'static str> {
//         let result = diesel::insert_into(users::table)
//             .values(create_request)
//             .execute(conn);

//         match result {
//             Ok(_) => {
//                 return Ok(());
//             }
//             Err(reason) => {
//                 // log the error.
//                 return Err("could not create the user");
//             }
//         }
//     }
// }
