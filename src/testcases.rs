
#[derive(Debug)]
pub struct TestCase {
    pub name: String,
}

#[derive(Debug)]
pub struct TestResult {
    pub name: String,
    pub success: bool,
    pub messages: Vec<String>,
}