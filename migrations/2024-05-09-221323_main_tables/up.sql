-- Your SQL goes here
CREATE TABLE IF NOT EXISTS Topics (
    topic_id uuid PRIMARY KEY,
    name TEXT,
    description TEXT
);

CREATE TABLE IF NOT EXISTS Users (
    user_id uuid PRIMARY KEY,
    username TEXT
);

CREATE TABLE IF NOT EXISTS Messages (
    message_id uuid PRIMARY KEY,
    topic_id uuid,
    user_id uuid,
    content TEXT,
    sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (topic_id) REFERENCES Topics(topic_id),
    FOREIGN KEY (user_id) REFERENCES Users(user_id)
);