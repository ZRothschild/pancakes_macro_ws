pub mod tool;

use hello_macro::HelloMacro;
use tool::help;

impl HelloMacro for help::Pancakes  {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}