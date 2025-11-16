use dioxus::prelude::*;
use crate::components::Input;   // import only Input

#[component]
pub fn Capture () -> Element {
    rsx! {
        div {
            input {
            class: "input",
        }
        }
    }
}
