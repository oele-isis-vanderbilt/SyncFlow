-- Your SQL goes here
CREATE TABLE IF NOT EXISTS create_room_actions (
    id SERIAL PRIMARY KEY,
    room_name TEXT NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS list_rooms_actions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    listed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS generate_token_actions (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    token_identity TEXT NOT NULL,
    token_room TEXT NOT NULL,
    generated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS delete_room_actions (
    id SERIAL PRIMARY KEY,
    room_name TEXT NOT NULL,
    user_id INT NOT NULL,
    deleted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id)
)
