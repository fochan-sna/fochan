-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE IF NOT EXISTS Topics (
    topic_id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Users (
    user_id uuid PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS Messages (
    message_id SERIAL PRIMARY KEY,
    topic_id uuid NOT NULL,
    user_id uuid NOT NULL,
    content TEXT NOT NULL,
    sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (topic_id) REFERENCES Topics(topic_id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
);
