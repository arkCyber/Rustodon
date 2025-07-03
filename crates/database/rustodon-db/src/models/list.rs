pub struct List {
    pub id: i64,
    pub title: String,
    pub replies_policy: Option<String>,
    pub exclusive: Option<bool>,
    pub account_id: i64,
    pub created_at: Option<chrono::NaiveDateTime>,
}
