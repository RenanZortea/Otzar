use dioxus::prelude::*;
use ui::Capture;

#[component]
pub fn Home() -> Element {
    rsx! {
        Capture { }
    }
}
