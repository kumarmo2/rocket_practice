-- Your SQL goes here
create table Users(
 id int auto_increment,
 name varchar(100) not null,
 age int not null,
 email varchar(100) not null unique,
 primary key(id)
);