-- Your SQL goes here
create table users(
 id int auto_increment,
 name varchar(100) not null,
 age int not null,
 email varchar(100) not null unique,
 password varchar(100) not null,
 primary key(id)
);