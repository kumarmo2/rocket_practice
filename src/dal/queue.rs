use crate::models::Queue;

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

pub fn get_queues_from_user_ids(member_id: &[i32], conn: &MysqlConnection) -> Option<Vec<Queue>> {
    use crate::schema::queues::dsl::*;

    let result = queues.filter(user_id.eq_any(member_id)).load(conn);

    match result {
        Ok(list) => Some(list),
        Err(reason) => {
            println!("some error, reason: {}", reason);
            None
        }
    }
}
