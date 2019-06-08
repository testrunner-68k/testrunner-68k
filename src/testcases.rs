
#[derive(Clone, Debug, PartialEq)]
pub struct TestCase {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestResult {
    pub name: String,
    pub success: bool,
    pub messages: Vec<String>,
}