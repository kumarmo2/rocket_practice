use crate::dtos::CreateUserRequest;
use crate::models::User;
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

pub fn create_from_request(
    create_request: &CreateUserRequest,
    hashed_pass: &str,
    conn: &MysqlConnection,
) -> Result<(), &'static str> {
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
            return Ok(());
        }
        Err(_) => {
            // log the error.
            return Err("could not create the user");
        }
    }
}

pub fn get_by_email(email_from_request: &str, conn: &MysqlConnection) -> Option<User> {
    use crate::schema::users::dsl::*;
    let results = users
        .filter(email.eq(email_from_request))
        .limit(1)
        .load::<User>(conn);
    match results {
        Ok(mut list) => {
            if list.len() < 1 {
                return None;
            } else {
                return Some(list.remove(0));
            }
        }
        Err(reason) => {
            println!("could not fetch user, reason: {}", reason);
            return None;
        }
    }
}
