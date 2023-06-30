use perseus::prelude::*;
use sycamore::web::SsrNode;

#[rx(alias = "TodoListStateRx")]
struct TodoListState {
    todos: Vec<String>,
    new_todo: String,
}

#[engine_only_fn]
async fn get_build_state(_info: StateGeneratorInfo<()>) -> TodoListState {
    TodoListState {
        todos: vec![
            "Grocery shoppig".to_string(),
            "Exercise for 30 minutes every morning".to_string(),
        ],
        new_todo: "".to_string(),
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Test App"}
        link(rel="stylesheet", href=".perseus/static/styles.css") {}
    }
}

#[engine_only_fn]
pub fn get_template<G: Html>() -> Template<G> {
    Template::build("index")
        .build_state_fn(get_build_state)
        // .view_with_state(todo_list_view)
        .head(head)
        .build()
}
