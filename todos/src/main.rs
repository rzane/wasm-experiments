extern crate stdweb;

use stdweb::web::{
    INode,
    document,
};

use stdweb::web::event::{
    ClickEvent
};

use stdweb::web::IEventTarget;
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryInto;

fn main() {
    stdweb::initialize();

    let button = document().query_selector("#submit").unwrap();

    button.add_event_listener(move |_: ClickEvent| {
        let input: InputElement = document().query_selector("#new-todo").unwrap().try_into().unwrap();
        let value = input.value().into_string().unwrap();
        append_todo(&value);
    });

    append_todo("Do the laundry!");
    append_todo("Run the dishwasher");

    stdweb::event_loop();
}

fn append_todo(text: &str) {
    let doc = document();
    let list = doc.query_selector("#todos").unwrap();

    let item = doc.create_element("li");
    let remove = doc.create_element("button");

    item.set_text_content(text);
    remove.set_text_content("Remove");

    item.append_child(&remove);
    list.append_child(&item);

    remove.add_event_listener(move |_: ClickEvent| {
        list.remove_child(&item).is_ok();
    });
}
