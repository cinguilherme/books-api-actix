drop table if exists book;
drop table if exists author;

create table book (
  id serial primary key,
  title varchar(100) not null,
  pages int not null,
  chapters int
);

create table author (
  id serial primary key,
  name varchar(100) not null
);
