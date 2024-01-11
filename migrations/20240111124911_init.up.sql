-- Add up migration script here

CREATE TABLE
    IF NOT EXISTS users (
        id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        name VARCHAR(255),
        email VARCHAR(255) UNIQUE,
        society VARCHAR(255),
        category VARCHAR(50)
    );

CREATE TABLE
    IF NOT EXISTS residents(
        resident_id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        name VARCHAR(255),
        email VARCHAR(255) UNIQUE,
        pfp_url VARCHAR(255),
        about_me VARCHAR(255),
        phone_no VARCHAR(20),
        flat_no INT,
        building VARCHAR(255),
        society VARCHAR(255)
    );
    
CREATE TABLE
    IF NOT EXISTS securities (
        security_id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        name VARCHAR(255),
        email VARCHAR(255) NOT NULL UNIQUE,
        pfp_url VARCHAR(255),
        phone_no VARCHAR(20),
        badge_id VARCHAR(255),
        society VARCHAR(255)
    );

CREATE TABLE
    IF NOT EXISTS visitors (
        visitor_id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        name VARCHAR(255),
        email VARCHAR(255) NOT NULL UNIQUE,
        phone_no VARCHAR(20),
        host_flat INT,
        host_building VARCHAR(255),
        host_society VARCHAR(255),
        badge_id VARCHAR(255),
        uid VARCHAR(255),
        otp VARCHAR(30)
    );

CREATE TABLE
    IF NOT EXISTS visitor_logs (
        log_id INT PRIMARY KEY NOT NULL AUTO_INCREMENT,
        name VARCHAR(255),
        phone_no VARCHAR(20),
        host_flat INT,
        host_building VARCHAR(255),
        host_society VARCHAR(255),
        entry DATETIME(6)
    );