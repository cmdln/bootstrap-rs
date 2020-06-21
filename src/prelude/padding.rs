use crate::props::IntoBsClass;

#[derive(Debug, Clone, PartialEq)]
pub struct Padding(pub super::Edge, pub usize);

impl IntoBsClass for Padding {
    fn as_classname(&self) -> String {
        format!("p{}-{}", self.0, self.1)
    }
}
