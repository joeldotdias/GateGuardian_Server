CREATE TABLE IF NOT EXISTS societies(
	society_id INT PRIMARY KEY AUTO_INCREMENT,
    society_name VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS users(
	user_id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    category VARCHAR(20) CHECK (category IN ('admin', 'resident', 'security')),
    society_id INT,
    FOREIGN KEY(society_id) REFERENCES societies(society_id)
);