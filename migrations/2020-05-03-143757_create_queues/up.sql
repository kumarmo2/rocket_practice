-- Your SQL goes here
create table queues(
id int auto_increment primary key,
user_id int not null,
queue_name varchar(100) not null unique,
constraint queue_user_exists_constraint
foreign key(user_id) references users(id)
);