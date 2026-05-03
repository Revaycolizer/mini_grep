use colored::Colorize;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn search_in_file(
    needle: &str,
    content: BufReader<File>,
    ignore_case: bool,
    line_numbers: bool,
    count_matches: bool,
    invert_match: bool,
) -> std::result::Result<(), Box<dyn Error>> {
    let pattern = if ignore_case {
        format!("(?i){}", regex::escape(needle))
    } else {
        regex::escape(needle)
    };

    let re = Regex::new(&pattern)?;

    let mut count = 0;

    for (i, line) in content.lines().enumerate() {
        let line = line?;

        let is_match = re.is_match(&line);
        let should_print = is_match ^ invert_match;

        if should_print {
            count += 1;

            if count_matches {
                continue;
            }

            let output = if !invert_match {
                re.replace_all(&line, |caps: &regex::Captures| {
                    caps[0].red().bold().to_string()
                })
                .to_string()
            } else {
                line.clone()
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

    Ok(())
}
