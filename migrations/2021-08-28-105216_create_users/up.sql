CREATE TABLE IF NOT EXISTS users(
    id            bigserial PRIMARY KEY,
    username      varchar(64) NOT NULL,
    email         varchar(64) NOT NULL,
    password_hash varchar(128)  NOT NULL,
    avatar_url    text,
    quot          text
);

CREATE UNIQUE INDEX IF NOT EXISTS user_index_id ON users (id);
CREATE UNIQUE INDEX IF NOT EXISTS user_index_username ON users (username);
CREATE UNIQUE INDEX IF NOT EXISTS user_index_email ON users (email);