pub fn wrap_text(s: String, max_length: usize) -> String {
    // Divides the text in max_length sentences joined with a \n
    let mut result = String::new();
    let mut current_line = String::new();
    let mut current_line_length = 0;

    for word in s.split_whitespace() {
        if word.len() > max_length {
            // If the word is longer than the max length, we split it
            let mut word = word.to_string();
            while word.len() > max_length {
                let (first, second) = word.split_at(max_length);
                current_line.push_str(first);
                current_line.push_str("\n");
                current_line_length = 0;
                word = second.to_string();
            }
            current_line.push_str(&word);
            current_line_length += word.len();
        } else if current_line_length + word.len() > max_length {
            // If the word doesn't fit in the current line, we start a new one
            result.push_str(&current_line.trim());
            result.push_str("\n");
            current_line = word.to_string();
            current_line_length = word.len();
        } else {
            // If the word fits in the current line, we add it
            current_line.push_str(word);
            current_line_length += word.len();
        }
        current_line.push_str(" ");
        current_line_length += 1;
    }

    result.push_str(&current_line.trim());
    result
}
