-- Your SQL goes here
CREATE TABLE customers(
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    document VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    second_name VARCHAR(255),
    person_type CHAR(1) NOT NULL,
    device_id VARCHAR(36)
);
