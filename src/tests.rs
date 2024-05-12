#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use crate::{GetMessagesResponse, models, PostMessageRequest, rocket};
    use uuid::Uuid;

    #[test]
    fn test_get_user_id() {
        let client = Client::tracked(rocket()).expect("Error creating client");
        let response = client.get("/get_user_id").dispatch();

        assert_eq!(response.status(), Status::Created);

        let _value: models::User = match serde_json::from_str(&response.into_string().unwrap()) {
            Ok(value) => value,
            Err(err) => panic!("User deserialization failed, some fields are wrong/missing: {}", err)
        };
    }

    #[test]
    fn test_get_topics() {
        use crate::GetTopicsResponse;
        
        let client = Client::tracked(rocket()).expect("Error creating client");
        let response = client.get("/get_topics").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let value: GetTopicsResponse = serde_json::from_str(&response.into_string().unwrap()).unwrap();

        let correct_response = GetTopicsResponse {
            topics: vec![
                models::Topic {
                    topic_id: Uuid::parse_str("b1074c65-6006-4858-a8b0-f6ff98b7fe03").unwrap(),
                    name: "Cooking".to_string(),
                    description: "Topic for cooking".to_string(),
                },
                models::Topic {
                    topic_id: Uuid::parse_str("9e1c2867-2ccb-4cef-9066-3b0c96a2d06a").unwrap(),
                    name: "Cars".to_string(),
                    description: "Topic about racing".to_string(),
                },
                models::Topic {
                    topic_id: Uuid::parse_str("f8e3f824-634a-483f-955f-86c998149cab").unwrap(),
                    name: "Anime".to_string(),
                    description: "Anime is life".to_string(),
                },
            ],
        };

        assert_eq!(correct_response, value);
    }

    #[test]
    fn test_post_and_get_message() {
        let client = Client::tracked(rocket()).expect("Error creating client");
        let user_response = client.get("/get_user_id").dispatch();
        let user: models::User = serde_json::from_str(&user_response.into_string().unwrap()).unwrap();

        let post_message_request = PostMessageRequest {
            user_id: user.user_id,
            topic_id: Uuid::parse_str("b1074c65-6006-4858-a8b0-f6ff98b7fe03").unwrap(),
            message: "test message".to_string(),
        };

        let post_message_response = client.post("/write_message").json(&post_message_request).dispatch();

        assert_eq!(post_message_response.status(), Status::Created);

        let get_messages_response = client.get("/get_messages?limit=1").dispatch();

        assert_eq!(get_messages_response.status(), Status::Ok);

        let messages: GetMessagesResponse = serde_json::from_str(&get_messages_response.into_string().unwrap()).unwrap();
        let message = messages.messages.get(0).unwrap();

        assert_eq!(message.content, post_message_request.message);
        assert_eq!(message.username, user.username);
    }
}