#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum Blank {
    #[default]
    Vertical = 0,
    VerticalHorizontal = 1,
}
