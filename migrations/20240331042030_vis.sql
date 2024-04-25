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