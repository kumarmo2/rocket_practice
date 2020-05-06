table! {
    messages (id) {
        id -> Integer,
        room_id -> Integer,
        sender_id -> Integer,
        content -> Text,
    }
}

table! {
    queues (id) {
        id -> Integer,
        user_id -> Integer,
        queue_name -> Varchar,
    }
}

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
    roomsubscribers (member_id, room_id) {
        member_id -> Integer,
        room_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        age -> Nullable<Integer>,
        email -> Varchar,
        password -> Varchar,
    }
}

joinable!(messages -> rooms (room_id));
joinable!(messages -> users (sender_id));
joinable!(queues -> users (user_id));
joinable!(rooms -> users (creator_user_id));
joinable!(roomsubscribers -> rooms (room_id));
joinable!(roomsubscribers -> users (member_id));

allow_tables_to_appear_in_same_query!(
    messages,
    queues,
    rooms,
    roomsubscribers,
    users,
);
