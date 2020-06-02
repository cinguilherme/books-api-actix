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

insert into book (title, pages, chapters)
values ('Crime and Passion', 150, 10);

insert into book (title, pages, chapters)
values ('Functional Domain Modeling', 450, 18);
