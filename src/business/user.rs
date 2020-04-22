use crate::models::User;
use diesel::MysqlConnection;

pub fn get_user_by_id(id: i32, conn: &MysqlConnection) -> Result<User, &'static str> {
    User::get_by_id(id, conn)
}
