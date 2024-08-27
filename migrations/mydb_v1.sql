begin;

create table users (
	"id" serial primary key,
	"username" varchar(64) not null,
	"password" varchar(64) not null,
	"created_at" timestamp not null,
	"updated_at" timestamp not null
);

create table users_items (
	"id" serial primary key,
	"user_id" bigint not null,
	"item_id" bigint not null,
	"created_at" timestamp not null
);

create table items (
	"id" serial primary key,
	"name" varchar(255) not null,
	"description" varchar(255) not null,
	"created_at" timestamp not null,
	"updated_at" timestamp not null
);

commit;