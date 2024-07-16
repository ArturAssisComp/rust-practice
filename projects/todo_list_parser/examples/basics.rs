extern crate todo_list_parser;
use todo_list_parser::TodoList;

fn main() {
    let todos = TodoList::get_todos("examples/todos.txt");
    match todos {
        Ok(list) => println!("{:?}", list),
        Err(e) => {
            println!("{}", e.to_string());
            println!("{:?}", e)
        }
    }
}
