#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn HpLogo() -> Element {
    rsx! {
        button {
            class: "btn btn-ghost text-2xl no-animation cursor-default font-harry_p gap-1",
            for (i, c) in "Life Planner".chars().enumerate() {
                p {
                    class: [
                        "animate-[hp_2s_1s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.3s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.6s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.2s_ease-in-out_infinite]",
                        "",
                        "animate-[hp_2s_0.1s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.9s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.5s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.1s_ease-in-out_infinite]",
                        "animate-[hp_2s_1s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.2s_ease-in-out_infinite]",
                        "animate-[hp_2s_0.1s_ease-in-out_infinite]",
                    ][i],
                    "{c}"
                }
            }
        }
    }
}
