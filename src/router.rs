#![allow(non_snake_case)]

use crate::{components::navbar::NavigationBar, pages::todo::TodoList};
use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::DailyPlanner {})]
    DailyPlanner {},
    #[route("/weekly")]
    WeeklyPlanner {},
    #[route("/yearly")]
    YearlyPlanner {},
    #[route("/life")]
    LifePlanner {},
    #[route("/expense")]
    ExpenseManager {},
    #[route("/settings")]
    SettingsPage {},
    #[route("/login")]
    SignInPage {},
}

fn DailyPlanner() -> Element {
    component_with_navbar(rsx! { TodoList {} })
}

fn WeeklyPlanner() -> Element {
    component_with_navbar(rsx! { div {"Weekly Planner"} })
}

fn YearlyPlanner() -> Element {
    component_with_navbar(rsx! { div {"Yearly Planner"} })
}

fn LifePlanner() -> Element {
    component_with_navbar(rsx! { div {"Life Planner"} })
}

fn ExpenseManager() -> Element {
    component_with_navbar(rsx! { div {"Expense Manager"} })
}

fn SettingsPage() -> Element {
    component_with_navbar(rsx! { div {"Settings Page"} })
}

fn SignInPage() -> Element {
    component_with_navbar(rsx! { div {"Sign In Page"} })
}

fn component_with_navbar(component: Element) -> Element {
    rsx! {
        div {
            div {
                class: "absolute top-0 h-screen w-screen flex justify-center py-20",
                {component}
            },
            NavigationBar {},
        }
    }
}
