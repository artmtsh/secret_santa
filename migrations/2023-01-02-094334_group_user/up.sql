-- Your SQL goes here
CREATE TYPE user_role AS ENUM ('user', 'admin');
CREATE TABLE group_user (
  group_id int REFERENCES groups(id) ON DELETE CASCADE NOT NULL,
  user_id int REFERENCES users(id) ON DELETE CASCADE NOT NULL,
  user_role user_role DEFAULT 'user' NOT NULL,
  ward_id int REFERENCES users(id) ON DELETE CASCADE,
  primary key(group_id, user_id)
);