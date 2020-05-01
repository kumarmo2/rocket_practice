-- Your SQL goes here
create table roomsubscribers (
	-- id int auto_increment,
    member_id int not null,
    room_id int not null,
    primary key(member_id, room_id),
    constraint room_constraint
    foreign key(room_id) references rooms(id),
    constraint member_constraint
    foreign key(member_id) references users(id)
);
