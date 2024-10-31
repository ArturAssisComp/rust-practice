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
    dbg!(file_content);

    Ok(())
}
