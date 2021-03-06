use crate::dal::last_insert_id;
use crate::dtos::CreateUserRequest;
use crate::models::User;
use diesel::dsl::select;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error;

pub fn get_by_id(id_input: i32, conn: &MysqlConnection) -> Result<User, &'static str> {
    use crate::schema::users::dsl::*;
    let results: Result<Vec<User>, Error> = users.filter(id.eq(id_input)).load::<User>(conn);
    match results {
        Ok(mut list) => {
            if list.len() < 1 {
                return Err("No user found");
            } else {
                return Ok(list.remove(0));
            }
        }
        Err(reason) => {
            // Log the error
            println!("could not fetch user, reason: {}", reason);
            return Err("Error while fetching results");
        }
    }
}

pub fn get_by_email(email_input: &str, conn: &MysqlConnection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;

    // Since, diesel has already been injected in all the layers of application because of how rocket
    // provides out of the box solution for DbConnections, which is kinda sad, we will return diesel::error::Error,
    // as it can let us make better decisisions in upper layers.
    users.filter(email.eq(email_input)).get_result::<User>(conn)
}

pub fn create_from_request(
    create_request: &CreateUserRequest,
    hashed_pass: &str,
    conn: &MysqlConnection,
) -> Result<i32, &'static str> {
    use crate::schema::users::dsl::*;

    let result = diesel::insert_into(users)
        .values((
            name.eq(&create_request.name),
            age.eq(create_request.age),
            email.eq(&create_request.email),
            password.eq(hashed_pass),
        ))
        .execute(conn);

    match result {
        Ok(_) => {
            // TODO: Need to fetch the id from the first call only.
            let ids = select(last_insert_id).load(conn).unwrap();
            return Ok(ids[0]);
        }
        Err(_) => {
            // log the error.
            return Err("could not create the user");
        }
    }
}
