use perseus::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::*;
use web_sys::Event;

// #[rx(alias = "TodoListStateRx")]
// struct TodoListState {
//     todos: Vec<String>,
//     new_todo: String,
// }

// fn header<G: Html>(cx: Scope) -> View<G> {
//     view! { cx,
//         h1 { "A simple Reactive Todo App"}
//     }
// }

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        // Don't worry, there are much better ways of styling in Perseus!
        div(style = "display: flex; flex-direction: column; justify-content: center; align-items: center; height: 95vh;") {
            h1 { "Welcome to Perseus!" }
        }
    }
}

// #[engine_only_fn]
// async fn get_build_state(_info: StateGeneratorInfo<()>) -> TodoListState {
//     TodoListState {
//         todos: vec![
//             "Grocery shoppig".to_string(),
//             "Exercise for 30 minutes every morning".to_string(),
//         ],
//         new_todo: "".to_string(),
//     }
// }

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Test App"}
        link(rel="stylesheet", href=".perseus/static/styles.css") {}
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index").view(index_page).head(head).build()
}
