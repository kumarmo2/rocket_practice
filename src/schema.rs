table! {
    rooms (id) {
        id -> Integer,
        name -> Varchar,
        creator_user_id -> Integer,
        url_identifier -> Varchar,
        is_public -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        age -> Integer,
        email -> Varchar,
    }
}

joinable!(rooms -> users (creator_user_id));

allow_tables_to_appear_in_same_query!(
    rooms,
    users,
);
