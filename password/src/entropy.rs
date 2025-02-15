#[derive(Default, Clone)]
struct PasswordParams {
    len: i32,
    has_lowercase: bool,
    has_uppercase: bool,
    has_digits: bool,
    has_symbols: bool
}

fn set_params(password: &str) -> PasswordParams {
    let mut params = PasswordParams::default();
    
    params.len = password.len() as i32;
    for ch in password.chars() {
        if ch.is_lowercase() { params.has_lowercase = true; }
        else if ch.is_uppercase() { params.has_uppercase = true; }
        else if ch.is_digit(10) { params.has_digits = true; }
        else { params.has_symbols = true; }
    }

    params
}

fn calculate_charset_size(params: PasswordParams) -> i32 {
    let mut charset_size = 0;

    if params.has_lowercase { charset_size += 26; }
    if params.has_uppercase { charset_size += 26; }
    if params.has_digits { charset_size += 10; }
    if params.has_symbols { charset_size += 32; }

    charset_size
}

pub fn calculate_entropy(password: &str) -> f64 {
    let params = set_params(password);
    let charset_size = calculate_charset_size(params.clone());
    if charset_size == 0 || params.len == 0 {
        return 0.0
    }

    let entropy = (charset_size as f64).log2() * params.len as f64;
    entropy
}
