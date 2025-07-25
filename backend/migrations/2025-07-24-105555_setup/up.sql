-- Your SQL goes here
CREATE TABLE `reminders`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`todo_id` INTEGER NOT NULL,
	`reminder` TIMESTAMP NOT NULL,
	FOREIGN KEY (`todo_id`) REFERENCES `todos`(`id`)
);

CREATE TABLE `users`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`username` TEXT NOT NULL,
	`password` TEXT NOT NULL
);

CREATE TABLE `todos`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`user_id` INTEGER NOT NULL,
	`title` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`due` TIMESTAMP,
	`repeat` TEXT NOT NULL,
	`completed` BOOL NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);

