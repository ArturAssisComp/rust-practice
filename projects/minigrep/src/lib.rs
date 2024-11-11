use std::{env, error::Error, fs};

pub struct ParsedMainArgs {
    query: String,
    file_path: String,
    ignore_case: bool,
}
impl ParsedMainArgs {
    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let binding = args
            .next()
            .expect("The name of the program is expected as the first argument");
        let filename = binding
            .rsplit(|c| c == '\\' || c == '/')
            .next()
            .expect("The name of the program is expected as the first argument");

        let Some(query) = args.next() else {
            return Err(format!(
                "Usage (query not found): {filename} <query> <file_path>"
            ));
        };

        let Some(file_path) = args.next() else {
            return Err(format!(
                "Usage (file path not found): {filename} <query> <file_path>"
            ));
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Self {
            query: query,
            file_path: file_path,
            ignore_case,
        })
    }
}

pub fn run(parsed_main_args: &ParsedMainArgs) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(parsed_main_args.file_path.as_str())?;
    if parsed_main_args.ignore_case {
        for (n, found_line) in
            search_case_insensitive(parsed_main_args.query.as_str(), &file_content)
        {
            println!("({}): \"{found_line}\"", n + 1);
        }
    } else {
        for (n, found_line) in search(parsed_main_args.query.as_str(), &file_content) {
            println!("({}): \"{found_line}\"", n + 1);
        }
    }
    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> impl Iterator<Item = (usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(move |(_, line)| line.contains(query))
}
fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> impl Iterator<Item = (usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(move |(_, line)| line.to_lowercase().contains(query.to_lowercase().as_str()))
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
                    "Should return one result (case sensitive)",
                    "Duct",
                    concat!(
                        "Rust:\n",
                        "safe, fast, productive.\n",
                        "Pick three.\n",
                        " this is a dDuctt \n"
                    ),
                    vec![" this is a dDuctt "],
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
                    search(query, contents).map(|v| v.1).collect::<Vec<&str>>(),
                    "{}",
                    description
                );
            }
        }
    }
    mod search_case_insensitive {
        use super::*;

        #[test]
        fn should_match_return_search_case_insensitive() {
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
                    "Should return two results (case insensitive)",
                    "DuCt",
                    concat!(
                        "Rust:\n",
                        "safe, fast, prodUctive.\n",
                        "Pick three.\n",
                        " this is a dDucTt \n"
                    ),
                    vec!["safe, fast, prodUctive.", " this is a dDucTt "],
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
                    search_case_insensitive(query, contents)
                        .map(|v| v.1)
                        .collect::<Vec<&str>>(),
                    "{}",
                    description
                );
            }
        }
    }
}
