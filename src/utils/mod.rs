use rand::{distributions::Alphanumeric, Rng};

pub fn generate_string(string_length: usize) -> String {
    let code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(string_length)
        .map(char::from)
        .collect();
    code
}

pub fn hash_email(email: Option<String>) -> Option<String> {
    match email {
        Some(e) => {
            if let Some(at_pos) = e.find('@') {
                let (name, domain) = e.split_at(at_pos);
                if name.len() > 3 {
                    let visible_part = &name[0..3];
                    let hash_length = 5.min(name.len() - 3);
                    let hashed_part = "x".repeat(hash_length);
                    let remaining_part = &name[(3 + hash_length)..];
                    return Some(format!("{}{}{}{}", visible_part, hashed_part, remaining_part, domain));
                }
                return Some(e);
            }
            None
        }
        None => None,
    }
}
