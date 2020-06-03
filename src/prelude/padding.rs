#[derive(Debug, Clone, PartialEq)]
pub struct Padding(pub super::Edge, pub usize);

impl super::BootstrapClass for Padding {
    fn as_classname(&self) -> String {
        format!("p{}-{}", self.0, self.1)
    }
}
