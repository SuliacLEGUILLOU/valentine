use uuid::Uuid;

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        assert_eq!(generate_id().len(), 36);
    }
}