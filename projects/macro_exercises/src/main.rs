macro_rules! hello_parser {
    (HELLO $string:tt) => {
        concat!("HELLO ", stringify!($string))
    };
    (HELLO $string:tt!) => {
        concat!("HELLO ", stringify!($string), "!")
    };
}

macro_rules! unordered_html_list {
    ($($element:expr), *) => {
        concat!("<ul>\n", $(concat!("\t<li>", $element, "</li>\n"), )* "</ul>")
    };
}

fn main() {
    println!(hello_parser!(HELLO world));
    println!(unordered_html_list!(1, 2, 3));
    println!(unordered_html_list!("element 1", 'h', 123));
}
