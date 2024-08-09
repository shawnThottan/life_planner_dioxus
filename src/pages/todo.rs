#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    title: String,
    description: String,
    completed: bool,
    id: u64,
}

#[component]
pub fn TodoList() -> Element {
    let mut todos = use_signal(|| {
        vec![
            Todo {
                title: "Buy Milk".to_string(),
                description: "When you go to the supermarket".to_string(),
                completed: true,
                id: 1,
            },
            Todo {
                title: "Buy Bread".to_string(),
                description: "When you go to the supermarket".to_string(),
                completed: false,
                id: 2,
            },
            Todo {
                title: "Buy Chocolates".to_string(),
                description: "When you go to the supermarket".to_string(),
                completed: false,
                id: 3,
            },
        ]
    });

    rsx! {
        div{
            class: "flex flex-col w-full items-center",
            div {
                class: "w-full",
                {
                    todos.cloned().iter().enumerate().map(|(i, todo)| rsx! {
                        div {
                            class: "bg-base-200 m-3 flex justify-between items-center rounded-lg",
                            div {
                                class: "collapse",
                                input {
                                    "name": "todo-element",
                                    r#type: "checkbox",
                                },
                                div {
                                    class: "collapse-title text-xl font-medium",
                                    div {
                                        class: "form-control",
                                        label {
                                            class: "label cursor-pointer",
                                            span {
                                                class: ["label-text", if todo.completed { "line-through" } else {""}].join(" "),
                                                "{todo.title}"
                                            }
                                        }
                                    }
                                },
                                div {
                                    class: "collapse-content",
                                    p {
                                        "{todo.description}"
                                    }
                                }
                            }
                            input {
                                r#type: "checkbox",
                                checked: "{todo.completed}",
                                class: "checkbox m-3",
                                onclick: move |_| {
                                    let mut todolist = todos.cloned();
                                    let mut todo = todolist.remove(i);
                                    todo.completed = !todo.completed;
                                    todolist.insert(i, todo);
                                    todos.set(todolist);
                                }
                            }
                        }
                    })
                }
            }
        }
    }
}
