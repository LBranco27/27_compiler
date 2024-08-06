use std::collections::HashSet;

// Define the permitted ID characters
pub fn permitted_id_characters() -> HashSet<char> {
    ['_', '-'].iter().cloned().collect()
}
