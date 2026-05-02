use colored::Colorize;
use regex::Regex;
pub fn search_in_file(
    needle: &str,
    content: &str,
    ignore_case: bool,
    line_numbers: bool,
    count_matches: bool,
    invert_match: bool,
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
        let is_match = re.is_match(line);
        let should_print = if invert_match { !is_match } else { is_match };
        if should_print {
            count += 1;

            if count_matches {
                continue;
            }
            let output = if !invert_match {
                re.replace_all(line, |caps: &regex::Captures| {
                    caps[0].red().bold().to_string()
                })
                .to_string()
            } else {
                line.to_string()
            };
            if line_numbers {
                println!("{}:{}", i + 1, output);
            } else {
                println!("{}", output);
            }
        }
    }

    if count_matches {
        println!("{}", count);
    }
}
