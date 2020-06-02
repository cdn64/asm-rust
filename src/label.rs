#[derive(Debug, Clone)]
pub struct Label(String);

impl Label {
    pub fn from(token: &str) -> Option<Self> {
        Some(Label(token.to_string()))
    }
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}
