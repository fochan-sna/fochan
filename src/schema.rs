// @generated automatically by Diesel CLI.

diesel::table! {
    messages (message_id) {
        message_id -> Uuid,
        topic_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        content -> Nullable<Text>,
        sent_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    topics (topic_id) {
        topic_id -> Uuid,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        username -> Nullable<Text>,
    }
}

diesel::joinable!(messages -> topics (topic_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    topics,
    users,
);
