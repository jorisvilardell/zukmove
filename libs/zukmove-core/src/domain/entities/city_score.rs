use serde::{Deserialize, Serialize};

/// Scores pour une ville, calculés à partir des tags des actualités.
/// Chaque métrique démarre à 1000 et ne descend jamais en dessous de 0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityScore {
    pub city: String,
    pub country: String,
    pub updated_at: String,
    pub quality_of_life: i32,
    pub safety: i32,
    pub economy: i32,
    pub culture: i32,
}

impl CityScore {
    /// Crée un nouveau CityScore avec les scores de base (1000).
    pub fn new(city: &str, country: &str) -> Self {
        Self {
            city: city.to_string(),
            country: country.to_string(),
            updated_at: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            quality_of_life: 1000,
            safety: 1000,
            economy: 1000,
            culture: 1000,
        }
    }

    /// Score total (somme des 4 métriques).
    pub fn total_score(&self) -> i32 {
        self.quality_of_life + self.safety + self.economy + self.culture
    }

    /// Applique les deltas d'un tag. Les scores ne descendent jamais en dessous de 0.
    pub fn apply_tag(&mut self, tag: &str) {
        let (qol, safety, economy, culture) = tag_deltas(tag);
        self.quality_of_life = (self.quality_of_life + qol).max(0);
        self.safety = (self.safety + safety).max(0);
        self.economy = (self.economy + economy).max(0);
        self.culture = (self.culture + culture).max(0);
        self.updated_at = chrono::Utc::now().format("%Y-%m-%d").to_string();
    }

    /// Applique tous les tags d'une news.
    pub fn apply_tags(&mut self, tags: &[String]) {
        for tag in tags {
            self.apply_tag(&tag.to_lowercase());
        }
    }
}

/// Retourne les deltas (quality_of_life, safety, economy, culture) pour un tag donné.
fn tag_deltas(tag: &str) -> (i32, i32, i32, i32) {
    match tag {
        "innovation" => (30, 20, 60, 5),
        "crime" => (-40, -80, -20, -10),
        "festival" => (20, 0, 10, 60),
        "economy" => (10, 0, 50, 0),
        "pollution" => (-50, -10, -5, -5),
        "tourism" => (20, 5, 30, 40),
        "education" => (30, 10, 20, 30),
        "health" => (40, 20, 10, 0),
        "sports" => (20, 5, 15, 30),
        "politics" => (0, -10, 10, 0),
        _ => (0, 0, 0, 0), // Unknown tags have no impact
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_city_score_starts_at_1000() {
        let score = CityScore::new("Paris", "France");
        assert_eq!(score.quality_of_life, 1000);
        assert_eq!(score.safety, 1000);
        assert_eq!(score.economy, 1000);
        assert_eq!(score.culture, 1000);
    }

    #[test]
    fn test_apply_innovation_tag() {
        let mut score = CityScore::new("Paris", "France");
        score.apply_tag("innovation");
        assert_eq!(score.quality_of_life, 1030);
        assert_eq!(score.safety, 1020);
        assert_eq!(score.economy, 1060);
        assert_eq!(score.culture, 1005);
    }

    #[test]
    fn test_score_never_below_zero() {
        let mut score = CityScore::new("TestCity", "TestCountry");
        // Apply crime many times
        for _ in 0..50 {
            score.apply_tag("crime");
        }
        assert_eq!(score.quality_of_life, 0);
        assert_eq!(score.safety, 0);
        assert_eq!(score.economy, 0);
        assert_eq!(score.culture, 0);
    }

    #[test]
    fn test_total_score() {
        let score = CityScore::new("Paris", "France");
        assert_eq!(score.total_score(), 4000);
    }
}
