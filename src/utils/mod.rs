use base64::encode;
use uuid::Uuid;

pub fn generate_v4_base64_uuid() -> String {
    let uuid = Uuid::new_v4();
    encode(uuid.to_string())
}
