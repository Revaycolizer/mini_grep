use regex::Regex;
pub fn search_in_file(
    needle: &str,
    content: &str,
    ignore_case: bool,
    line_numbers: bool,
    count_matches: bool,
) {
    let pattern = if ignore_case {
        format!("(?i){}", regex::escape(needle))
    } else {
        regex::escape(needle)
    };

    let re = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Invalid Pattern :{}", e);
            return;
        }
    };

    let mut count = 0;
    for (i, line) in content.lines().enumerate() {
        if re.is_match(line) {
            count += 1;
            if !count_matches {
                if line_numbers {
                    println!("{}:{}", 1 + i, line);
                } else {
                    println!("{}", line);
                }
            }
        }
    }

    if count_matches {
        println!("{}", count);
    }
}
