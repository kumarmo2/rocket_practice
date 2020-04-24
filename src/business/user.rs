use crate::models::User;
use diesel::MysqlConnection;
use crate::dal::user;

pub fn get_user_by_id(id: i32, conn: &MysqlConnection) -> Result<User, &'static str> {
    user::get_by_id(id, conn)
}
