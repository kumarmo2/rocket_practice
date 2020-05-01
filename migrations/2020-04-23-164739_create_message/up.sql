-- Your SQL goes here

create table messages(
    id int auto_increment primary key,
    room_id int not null,
    sender_id int not null,
    content text not null,
    constraint room_constraint_for_message
    foreign key(room_id) references rooms(id),
    constraint sender_constraint
    foreign key(sender_id) references users(id)
);
