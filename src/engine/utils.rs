use uuid::Uuid;

/**
 * Generate a uniq id for system resource
 * This is build as a function so it can easily be use for serde default value
 * TODO: This could be optimized a little bit by removing the '-' from the uuid
 */
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn test_empty_vec<T>(v: &Vec<T>) -> bool {
    v.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        assert_eq!(generate_id().len(), 36);
    }
}