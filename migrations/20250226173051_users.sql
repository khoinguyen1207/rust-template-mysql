-- Add migration script here
DROP TABLE IF EXISTS 'users';

CREATE TABLE users (
  user_id int NOT NULL AUTO_INCREMENT,
  username varchar(512) NOT NULL,
  amount decimal(15,2) DEFAULT 0.00,
  created_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (user_id)
)