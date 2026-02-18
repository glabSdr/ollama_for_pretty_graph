use serde::Serialize;

/// Extra configurations. Compatible with options in Ollama
#[derive(Debug, Serialize)]
pub struct Config {
    pub seed: Option<i32>,
    pub temperature: f32,
    pub top_p: f32,
    pub min_p: f32,
    pub top_k: i32,
    pub num_predict: Option<i32>,
    pub stop: Vec<String>
}

impl Default for Config {

    /// Generates a default configuration. Fields can be manually modified if needed.
    /// Values:
    /// ```
    /// seed: None,
    /// temperature: 0.7,
    /// top_p: 0.9,
    /// min_p: 0.0,
    /// top_k: 40,
    /// num_predict: None,
    /// stop: vec![]
    /// ```
    fn default() -> Self {
        Self {
            seed: None,
            temperature: 0.7,
            top_p: 0.9,
            min_p: 0.0,
            top_k: 40,
            num_predict: None,
            stop: vec![]
        }
    }
}

impl Config {
    /// Generates strict config for more accurate calculations.
    /// Values:
    /// ```
    /// seed: Some(42),
    /// temperature: 0.0,
    /// top_p: 0.9,
    /// min_p: 0.1,
    /// top_k: 20,
    /// num_predict: None,
    /// stop: vec![]
    /// ```
    pub fn strict() -> Self {
        Self {
            seed: Some(42),
            temperature: 0.0,
            top_p: 0.9,
            min_p: 0.1,
            top_k: 20,
            num_predict: None,
            stop: vec![]
        }
    }
}

#[cfg(test)]
 mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let _ = Config::default();
    }

    #[test]
    fn test_strict() {
        let _ = Config::strict();
    }
}