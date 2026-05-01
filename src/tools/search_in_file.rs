use regex::Regex;
pub fn search_in_file(needle: &str, content: &str, ignore_case: bool, line_numbers: bool) {
    let pattern = if ignore_case {
        format!("(?i){}", regex::escape(needle))
    } else {
        regex::escape(needle)
    };

    let pattern_matcher = Regex::new(&pattern).unwrap();

    for (i, line) in content.lines().enumerate() {
        if pattern_matcher.is_match(&line) {
            match line_numbers {
                true => {
                    println!("{} {}", 1 + i, line);
                }
                false => {
                    println!("{}", line);
                }
            }
        }
    }
}
