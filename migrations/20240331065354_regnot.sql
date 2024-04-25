CREATE table IF NOT EXISTS notices(
	notice_id int PRIMARY KEY AUTO_INCREMENT,
	title varchar(255),
    body varchar(255),
    society_id int,
    posted DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(society_id) REFERENCES societies(society_id)
);

CREATE table IF NOT EXISTS regulars(
	regular_id int PRIMARY KEY AUTO_INCREMENT,
	society_id int,
    name varchar(255),
    role varchar(255),
    resident_email varchar(255),
    entry varchar(10),
    FOREIGN KEY(society_id) REFERENCES societies(society_id),
    FOREIGN KEY(resident_email) REFERENCES residents(email)
);
