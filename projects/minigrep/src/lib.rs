use std::{error::Error, fs};

pub struct ParsedMainArgs<'a> {
    query: &'a str,
    file_path: &'a str,
}
impl<'a> ParsedMainArgs<'a> {
    pub fn query(&self) -> &str {
        self.query
    }
    pub fn file_path(&self) -> &str {
        self.file_path
    }
    pub fn build(args: &'a [String]) -> Result<Self, String> {
        if args.len() != 3 {
            return Err(format!(
                "Usage: {} <query> <file_path>",
                args[0]
                    .rsplit(|c| c == '\\' || c == '/')
                    .next()
                    .expect("The name of the program is expected as the first argument")
            ));
        }
        Ok(Self {
            query: &args[1],
            file_path: &args[2],
        })
    }
}

pub fn run(parsed_main_args: &ParsedMainArgs) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(parsed_main_args.file_path)?;
    for (n, found_line) in search(parsed_main_args.query(), &file_content) {
        println!("({}): \"{found_line}\"", n + 1);
    }
    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(move |(_, line)| line.contains(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod search {
        use super::*;

        #[test]
        fn should_match_return_search() {
            let test_cases = [
                (
                    "Should return empty result",
                    "not present",
                    concat!("Line 1 \n", "Line 2\n"),
                    Vec::<&str>::new(),
                ),
                (
                    "Should return empty result (query: \\n)",
                    "\n",
                    concat!("Line 1 \n", "Line 2\n"),
                    Vec::<&str>::new(),
                ),
                (
                    "Should return empty result (invalid substring)",
                    "safeduct",
                    concat!("Rust:\n", "safe, fast, productive.\n", "Pick three.",),
                    Vec::<&str>::new(),
                ),
                (
                    "Should return one result",
                    "duct",
                    concat!("Rust:\n", "safe, fast, productive.\n", "Pick three.",),
                    vec!["safe, fast, productive."],
                ),
                (
                    "Should return two results",
                    "duct",
                    concat!(
                        "Rust:\n",
                        "safe, fast, productive.\n",
                        "Pick three.\n",
                        " this is a dductt \n"
                    ),
                    vec!["safe, fast, productive.", " this is a dductt "],
                ),
                (
                    "Should match everything",
                    "",
                    concat!("Rust:\n", "safe, fast, productive.\n", "Pick three.",),
                    vec!["Rust:", "safe, fast, productive.", "Pick three."],
                ),
            ];

            for (description, query, contents, expected_result) in test_cases {
                assert_eq!(
                    expected_result,
                    search(query, contents).collect::<Vec<&str>>(),
                    "{}",
                    description
                );
            }
        }
    }
}
