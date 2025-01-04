-- Your SQL goes here
CREATE TABLE `vertices`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`payload` TEXT NOT NULL
);

CREATE TABLE `edges`(
	`start` INTEGER NOT NULL,
	`finish` INTEGER NOT NULL,
	`payload` TEXT NOT NULL,
	PRIMARY KEY(`start`, `finish`)
);

