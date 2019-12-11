use uuid::Uuid;

/**
 * Generate a uniq id for system resource
 * This is build as a function so it can easily be use for serde default value
 * TODO: This could be optimized a little bit by removing the '-' from the uuid
 */
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn empty_string() -> String {
    "".to_string()
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

    #[test]
    fn test_empty_string() {
        assert_eq!(empty_string().len(), 0);
    }

    #[test]
    fn test_test_empty_vec() {
        assert_eq!(test_empty_vec::<u32>(&vec![]), true);
        assert_eq!(test_empty_vec::<u32>(&vec![10, 5]), false);
        assert_eq!(test_empty_vec::<String>(&vec![]), true);
        assert_eq!(test_empty_vec::<String>(&vec![String::from("1234")]), false);
    }
}
