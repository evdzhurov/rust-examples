pub struct Song {
    pub title: String,
    pub artist: String,
    pub rating: i64,
}

impl Song {
    pub fn new(title: String, artist: String, rating: i64) -> Self {
        Self {
            title,
            artist,
            rating,
        }
    }
}
