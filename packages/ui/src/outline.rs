use dioxus::prelude::*;
use uuid::Uuid;
use crate::render_markdown;

#[derive(Clone, PartialEq, Copy, Debug)]
pub struct Block {
    pub id: Uuid,
    pub content: Signal<String>,
    pub children: Signal<Vec<Block>>,
    pub collapsed: Signal<bool>,
    // To manage focus
    pub is_editing: Signal<bool>, 
}

impl Block {
    pub fn new(text: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: Signal::new(text.to_string()),
            children: Signal::new(Vec::new()),
            collapsed: Signal::new(false),
            is_editing: Signal::new(false),
        }
    }
}

/// Actions that a block can request its parent to perform
#[derive(Clone, PartialEq)]
pub enum BlockAction {
    Split { id: Uuid, content_after: String },
    MergeUp { id: Uuid },
    Indent { id: Uuid },
    Outdent { id: Uuid },
    Next { id: Uuid },
    Prev { id: Uuid },
}

#[component]
pub fn BlockNode(
    block: Block, 
    // Callback to the parent to handle structural changes
    on_action: EventHandler<BlockAction>
) -> Element {
    let mut content = block.content;
    let mut is_editing = block.is_editing;
    let mut children = block.children;
    let mut collapsed = block.collapsed;

    rsx! {
        div { class: "block-wrapper",
            div { class: "block-row",
                // Bullet Point
                div { 
                    class: "bullet-container",
                    onclick: move |_| {
                        let current = *collapsed.read();
                        collapsed.set(!current);
                    },
                    div { class: "bullet" }
                }

                // Content Area
                div { class: "block-content",
                    if is_editing() {
                        textarea {
                            class: "block-editor",
                            value: "{content}",
                            autofocus: true,
                            oninput: move |e| content.set(e.value()),
                            onblur: move |_| is_editing.set(false),
                            onkeydown: move |e| {
                                match e.key() {
                                    Key::Enter if !e.modifiers().shift() => {
                                        // Split block
                                        on_action.call(BlockAction::Split { 
                                            id: block.id, 
                                            content_after: "".to_string() // Simplified split
                                        });
                                        // Prevent default enter behavior (newline)
                                        e.prevent_default(); 
                                    }
                                    Key::Tab => {
                                        e.prevent_default();
                                        if e.modifiers().shift() {
                                            on_action.call(BlockAction::Outdent { id: block.id });
                                        } else {
                                            on_action.call(BlockAction::Indent { id: block.id });
                                        }
                                    }
                                    Key::Backspace if content.read().is_empty() => {
                                        on_action.call(BlockAction::MergeUp { id: block.id });
                                        e.prevent_default();
                                    }
                                    Key::ArrowUp => {
                                         on_action.call(BlockAction::Prev { id: block.id });
                                    }
                                    Key::ArrowDown => {
                                         on_action.call(BlockAction::Next { id: block.id });
                                    }
                                    _ => {}
                                }
                            }
                        }
                    } else {
                        // Render Markdown when not editing
                        div { 
                            class: "rendered-markdown",
                            onclick: move |_| is_editing.set(true),
                            dangerous_inner_html: "{render_markdown(&content.read())}"
                        }
                    }
                }
            }

            // Recursive Children
            if !children.read().is_empty() && !collapsed() {
                div { class: "children-container",
                    for child in children.read().iter() {
                        BlockNode {
                            key: "{child.id}",
                            block: *child,
                            on_action: move |action| {
                                match action {
                                    // Handle INDENT (Child wants to become grandchild)
                                    BlockAction::Indent { id } => {
                                        let mut kids = children.write();
                                        if let Some(pos) = kids.iter().position(|b| b.id == id) {
                                            if pos > 0 {
                                                let node = kids.remove(pos);
                                                // Move into previous sibling
                                                let prev_sibling = &mut kids[pos - 1];
                                                prev_sibling.children.write().push(node);
                                                prev_sibling.collapsed.set(false); // Ensure expanded
                                                
                                                // FIX: Added 'mut' here
                                                if let Some(mut last_child) = prev_sibling.children.read().last().copied() {
                                                    last_child.is_editing.set(true);
                                                }
                                            }
                                        }
                                    }
                                    // Handle OUTDENT (Child wants to become sibling)
                                    BlockAction::Outdent { id } => {
                                        // Pass it up to *our* parent
                                        on_action.call(BlockAction::Outdent { id }); 
                                    }
                                    // Handle SPLIT (Enter key)
                                    BlockAction::Split { id, content_after: _ } => {
                                         let mut kids = children.write();
                                         if let Some(pos) = kids.iter().position(|b| b.id == id) {
                                             let mut new_block = Block::new("");
                                             new_block.is_editing.set(true);
                                             kids.insert(pos + 1, new_block);
                                         }
                                    }
                                    // Handle MERGE UP (Backspace on empty)
                                    BlockAction::MergeUp { id } => {
                                        let mut kids = children.write();
                                        if let Some(pos) = kids.iter().position(|b| b.id == id) {
                                            kids.remove(pos);
                                            if pos > 0 {
                                                kids[pos - 1].is_editing.set(true);
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
