-- Your SQL goes here

create table rooms (
    id int primary key auto_increment,
    name varchar(20) not null,
    creator_user_id int not null,
    url_identifier varchar(200) not null unique,
    is_public bool not null default true,
    constraint creator_constraint
    foreign key(creator_user_id) references users(id)
);