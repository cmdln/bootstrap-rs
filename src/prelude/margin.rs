#[derive(Debug, Clone, PartialEq)]
pub struct Margin(pub super::Edge, pub usize);

impl super::BootstrapClass for Margin {
    fn as_classname(&self) -> String {
        format!("m{}-{}", self.0, self.1)
    }
}
