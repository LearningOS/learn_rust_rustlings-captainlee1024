// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    if input.len() <= 0 {
        return "".to_string();
    }

    let mut tmp = input.clone();
    let mut items: Vec<char> = tmp.chars().collect();

    items[0] = items[0].to_uppercase().nth(0).unwrap();
    items.iter().collect()
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut w: Vec<String> = Vec::new();
    for c in words {
                let mut tmp = c.clone();
                let mut items: Vec<char> = tmp.chars().collect();
                items[0] = items[0].to_uppercase().nth(0).unwrap();
                let s: String= items.iter().collect();

                w.push(s);
    }
    w
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut ww = String::new();
    for c in words {
        if *c != " " {
            let mut tmp = c.clone();
            let mut items: Vec<char> = tmp.chars().collect();
            items[0] = items[0].to_uppercase().nth(0).unwrap();

            let s: String= items.iter().collect();
            ww += s.as_str();
        } else {
            ww += *c;
        }

    }

    ww
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
