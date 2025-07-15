-- Your SQL goes here
CREATE TABLE `todos`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`title` TEXT NOT NULL,
	`description` TEXT NOT NULL,
	`due` TIMESTAMP,
	`repeat` TEXT NOT NULL,
	`completed` BOOL NOT NULL
);

CREATE TABLE `reminders`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`todo_id` INTEGER NOT NULL,
	`reminder` TIMESTAMP NOT NULL,
	FOREIGN KEY (`todo_id`) REFERENCES `todos`(`id`)
);