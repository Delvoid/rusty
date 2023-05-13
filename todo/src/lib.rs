mod list {

    pub struct _Tasks {
        pub item: String,
    }
}

mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;

fn _lets_add_task() {
    let _task = list::_Tasks {
        item: String::from("Learn Rust"),
    };

    add_activity();
    items_completed::_remove_task();
}
