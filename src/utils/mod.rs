use base64::{decode, encode};
use uuid::Uuid;

pub fn generate_v4_base64_uuid() -> String {
    let uuid = Uuid::new_v4();
    encode(uuid.to_string())
}
