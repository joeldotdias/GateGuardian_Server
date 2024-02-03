-- This file contains the migration scripts to spin up the database

CREATE TABLE IF NOT EXISTS societies(
	society_id INT PRIMARY KEY AUTO_INCREMENT,
    society_name VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS users(
	user_id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255),
    email VARCHAR(255),
    category VARCHAR(20) CHECK (category IN ('admin', 'resident', 'security')),
    society_id INT,
    FOREIGN KEY(society_id) REFERENCES societies(society_id)
);

CREATE TABLE IF NOT EXISTS residents(
	resident_id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT,
    phone_no VARCHAR(15),
    flat_no INT,
    building VARCHAR(255),
    pfp_url VARCHAR(255),
    about_me VARCHAR(255),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS securities(
	security_id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT,
    phone_no VARCHAR(14),
    badge_id VARCHAR(255),
    pfp_url VARCHAR(255),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);

CREATE TABLE IF NOT EXISTS visitors(
	visitor_id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255),
    phone_no VARCHAR(15),
    resident_id INT,
    code VARCHAR(20),
    FOREIGN KEY(resident_id) REFERENCES residents(resident_id)
);

CREATE TABLE IF NOT EXISTS visitor_logs(
	log_id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255),
    phone_no VARCHAR(15),
    resident_id INT,
    entry DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(resident_id) REFERENCES residents(resident_id)
);
