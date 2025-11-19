use dioxus::prelude::*;
use crate::outline::{Block, BlockNode, BlockAction};

const OUTLINE_CSS: Asset = asset!("/assets/styling/outline.css");

#[component]
pub fn Capture() -> Element {
    // Initial State: One block
    let mut blocks = use_signal(|| vec![
        Block::new("Welcome to **Otzar** (Logseq Clone)"),
        Block::new("Press **Enter** to create a new block"),
        Block::new("Press **Tab** to indent"),
    ]);

    rsx! {
        document::Link { rel: "stylesheet", href: OUTLINE_CSS }
        
        div { 
            id: "capture-page",
            class: "outline-container",
            
            h1 { "Capture" }

            // Root Level Blocks
            for block in blocks.read().iter() {
                BlockNode {
                    key: "{block.id}",
                    block: *block,
                    on_action: move |action| {
                        // Handle Root Level actions
                        match action {
                            BlockAction::Split { id, .. } => {
                                let mut list = blocks.write();
                                if let Some(pos) = list.iter().position(|b| b.id == id) {
                                    let mut new_block = Block::new("");
                                    new_block.is_editing.set(true);
                                    list.insert(pos + 1, new_block);
                                }
                            }
                            BlockAction::MergeUp { id } => {
                                let mut list = blocks.write();
                                if let Some(pos) = list.iter().position(|b| b.id == id) {
                                    if pos > 0 {
                                        list.remove(pos);
                                        list[pos - 1].is_editing.set(true);
                                    }
                                }
                            }
                            BlockAction::Indent { id } => {
                                let mut list = blocks.write();
                                if let Some(pos) = list.iter().position(|b| b.id == id) {
                                    if pos > 0 {
                                        let node = list.remove(pos);
                                        let prev = &mut list[pos - 1];
                                        prev.children.write().push(node);
                                        prev.collapsed.set(false);
                                        
                                        // FIX: Added 'mut' here
                                        if let Some(mut last_child) = prev.children.read().last().copied() {
                                            last_child.is_editing.set(true);
                                        }
                                    }
                                }
                            }
                            // Root blocks cannot outdent further in this view
                            _ => {}
                        }
                    }
                }
            }
            
            // Helper to always have an input at bottom if list is empty
            if blocks.read().is_empty() {
                div {
                    class: "empty-state",
                    onclick: move |_| {
                        let mut new_block = Block::new("");
                        new_block.is_editing.set(true);
                        blocks.write().push(new_block);
                    },
                    "Click to add block"
                }
            }
        }
    }
}
