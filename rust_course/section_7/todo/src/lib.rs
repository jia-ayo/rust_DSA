mod list {
    pub struct Tasks {
        pub item: String,
    }

}

mod things_todo;
use crate::things_todo::add_activities;
use things_todo::items_completed;
use things_todo::items_completed::test::test;
fn lets_add_task() {
    let task = list::Tasks {
        item: String::from("Tasks"),
    };
    things_todo::add_activities();
    add_activities();
    items_completed::remove_task();
    test();
}
