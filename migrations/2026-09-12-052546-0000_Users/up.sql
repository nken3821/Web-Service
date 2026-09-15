-- Your SQL goes here
create table users (
    id int primary key auto_increment,
    username varchar(25) not null,
    password_hash varchar(50) not null,
    email varchar(30) not null,
    created_at datetime not null default current_timestamp,
    updated_at datetime not null default current_timestamp
        on update current_timestamp
)