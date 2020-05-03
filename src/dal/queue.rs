use diesel::prelude::*;
use diesel::MysqlConnection;

pub fn create(user_id_input: i32, queue_name_input: &str, conn: &MysqlConnection) -> Option<()> {
    use crate::schema::queues::dsl::*;

    match diesel::insert_into(queues)
        .values((user_id.eq(user_id_input), queue_name.eq(queue_name_input)))
        .execute(conn)
    {
        Ok(_) => Some(()),
        Err(reason) => {
            println!("insertion into queues table failed, reason: {}", reason);
            None
        }
    }
}
