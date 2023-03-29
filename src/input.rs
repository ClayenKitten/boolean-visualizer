pub mod text_input;
pub mod graph_input;
pub mod selector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum InputKind {
    #[default]
    Text = 0,
    Graph = 1,
}
