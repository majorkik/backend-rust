CREATE TABLE IF NOT EXISTS users(
    id            bigserial PRIMARY KEY,
    username      varchar(32) UNIQUE NOT NULL,
    email         varchar(64) UNIQUE NOT NULL,
    pass_hash   text NOT NULL,
    user_role text[] NOT NULL DEFAULT '{"user"}'
);

CREATE UNIQUE INDEX IF NOT EXISTS user_index_id ON users (id);
CREATE UNIQUE INDEX IF NOT EXISTS user_index_username ON users (username);
CREATE UNIQUE INDEX IF NOT EXISTS user_index_email ON users (email);