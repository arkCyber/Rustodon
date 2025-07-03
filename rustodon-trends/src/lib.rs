#[allow(dead_code)]
pub struct TrendsService {
    #[allow(dead_code)]
    pool: sqlx::PgPool,
    #[allow(dead_code)]
    cache: HashMap<String, (DateTime<Utc>, Vec<u8>)>, // Simple in-memory cache
}

impl TrendsService {
    fn calculate_tag_score(&self, _tag: &str, _history: &[TrendHistory]) -> f64 {
        0.0
    }
    fn calculate_status_score(&self, _status: &TrendingStatus) -> f64 {
        0.0
    }
}
