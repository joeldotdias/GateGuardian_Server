CREATE TABLE IF NOT EXISTS residents(
	resident_id INT PRIMARY KEY AUTO_INCREMENT,
    email VARCHAR(255),
    phone_no VARCHAR(15),
    flat_no INT,
    building VARCHAR(255),
    pfp_url VARCHAR(255),
    about_me VARCHAR(255),
    FOREIGN KEY(email) REFERENCES users(email)
);

CREATE TABLE IF NOT EXISTS securities(
	security_id INT PRIMARY KEY AUTO_INCREMENT,
    email VARCHAR(255),
    phone_no VARCHAR(15),
    badge_id VARCHAR(255),
    pfp_url VARCHAR(255),
    FOREIGN KEY(email) REFERENCES users(email)
);