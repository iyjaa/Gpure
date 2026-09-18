pub fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;

    for (i, c) in input.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 && (!prev_was_upper || {
                input.chars().nth(i + 1).map_or(false, |next| next.is_lowercase())
            }) {
                result.push('_');
            }
            result.extend(c.to_lowercase());
            prev_was_upper = true;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }
    result
}

pub fn to_camel_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in input.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else {
            if result.is_empty() {
                result.extend(c.to_lowercase());
            } else {
                result.push(c);
            }
        }
    }
    result
}

pub fn truncate(input: &str, max_len: usize, suffix: &str) -> String {
    if input.len() <= max_len {
        return input.to_string();
    }
    
    let suffix_len = suffix.len();
    if max_len <= suffix_len {
        return suffix[..max_len].to_string();
    }
    
    let cut_at = max_len - suffix_len;
    format!("{}{}", &input[..cut_at], suffix)
}

pub fn pad_start(input: &str, width: usize, pad_char: char) -> String {
    if input.len() >= width {
        return input.to_string();
    }
    let padding = width - input.len();
    format!("{}{}", pad_char.to_string().repeat(padding), input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_basic() {
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
    }

    #[test]
    fn test_to_snake_case_acronym() {
        assert_eq!(to_snake_case("getHTTPResponse"), "get_http_response");
    }

    #[test]
    fn test_to_snake_case_already_snake() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    fn test_to_camel_case_from_snake() {
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
    }

    #[test]
    fn test_to_camel_case_from_kebab() {
        assert_eq!(to_camel_case("my-project-name"), "myProjectName");
    }

    #[test]
    fn test_truncate_normal() {
        assert_eq!(truncate("Hello World", 8, "..."), "Hello...");
    }

    #[test]
    fn test_truncate_no_cut_needed() {
        assert_eq!(truncate("Hi", 10, "..."), "Hi");
    }

    #[test]
    fn test_pad_start_numbers() {
        assert_eq!(pad_start("5", 3, '0'), "005");
    }

    #[test]
    fn test_pad_start_no_padding_needed() {
        assert_eq!(pad_start("12345", 3, '0'), "12345");
    }
}
