pub mod text_input;
pub mod graph_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputKind {
    #[default]
    Text,
    Graph,
}
