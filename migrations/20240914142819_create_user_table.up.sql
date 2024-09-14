-- Add up migration script here
CREATE TYPE UserStatus AS ENUM (
  'ACTIVE',
  'INACTIVE'
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  password VARCHAR(255) NOT NULL,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  permission_system_setting BOOLEAN DEFAULT FALSE,
  permission_schedule BOOLEAN DEFAULT FALSE,
  permission_temporary_schedule BOOLEAN DEFAULT FALSE,
  permission_post_setting BOOLEAN DEFAULT FALSE,
  status UserStatus DEFAULT 'INACTIVE'
);



