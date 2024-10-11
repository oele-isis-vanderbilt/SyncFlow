-- Your SQL goes here


ALTER TABLE syncflow.users
ADD COLUMN first_name VARCHAR(255),
ADD COLUMN middle_name VARCHAR(255),
ADD COLUMN last_name VARCHAR(255),
ADD COLUMN organization VARCHAR(255),
ADD COLUMN job_role VARCHAR(255);
