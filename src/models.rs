// structs for messages and users

pub struct ChatMessage {
    pub channel_name: String,
    pub language: String,
    pub viewer_count: i32,
}

pub struct LanguajeResult {
    pub language: String,
    pub count: i32,
}

pub struct ChannelResult {
    pub channel_name: String,
    pub avg_viewers: f32,
}