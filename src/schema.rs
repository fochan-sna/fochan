// @generated automatically by Diesel CLI.

diesel::table! {
    messages (message_id) {
        message_id -> Int4,
        topic_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        sent_at -> Timestamp,
    }
}

diesel::table! {
    topics (topic_id) {
        topic_id -> Uuid,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Text,
    }
}

diesel::joinable!(messages -> topics (topic_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    topics,
    users,
);
