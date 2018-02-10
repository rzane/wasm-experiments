extern crate stdweb;

use stdweb::web::{
    INode,
    document
};

fn main() {
    stdweb::initialize();

    let header = document().query_selector("h1").unwrap();
    header.set_text_content("Hello from Rust!");
}
