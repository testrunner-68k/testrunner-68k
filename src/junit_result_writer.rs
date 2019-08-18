

use std::fs;
use std::io::Cursor;

use xml::writer::{EmitterConfig, XmlEvent, Result};

use super::testcases::{TestResult};

pub fn format_test_results(test_results: &Vec<TestResult>) -> Result<String> {

    let mut writer = EmitterConfig::new().perform_indent(true).create_writer(Cursor::new(Vec::new()));

    writer.write(XmlEvent::start_element("testsuite").attr("tests", &format!("{}", test_results.len())))?;

    for test_result in test_results {
        writer.write(XmlEvent::start_element("testcase").attr("name", &test_result.name))?;

        if !test_result.success {
            writer.write(XmlEvent::start_element("failure"))?;
            let failure_text = test_result.messages.join("\n");
            writer.write(XmlEvent::characters(&failure_text))?;
            writer.write(XmlEvent::end_element())?;
        }

        writer.write(XmlEvent::end_element())?;
    }

    writer.write(XmlEvent::end_element())?;

    let result = writer.into_inner().into_inner();

    Ok(String::from_utf8(result).unwrap())
}

pub fn write_test_results(test_results: &Vec<TestResult>, filename: &str) {

    let result_string = format_test_results(&test_results).unwrap();
    fs::write(filename, result_string).unwrap();
}

#[test]
fn format_junit_test_results_correctly() {
    let test_results = vec!(
        TestResult { name: String::from("test 1"), success: true, messages: vec!(String::from("test message 1"), String::from("test message 2")) },
        TestResult { name: String::from("test 2"), success: false, messages: vec!(String::from("test message 3")) }
    );

    let expected_result="<?xml version=\"1.0\" encoding=\"utf-8\"?>
<testsuite tests=\"2\">
  <testcase name=\"test 1\" />
  <testcase name=\"test 2\">
    <failure>test message 3</failure>
  </testcase>
</testsuite>";

    let result_string = format_test_results(&test_results).unwrap();
    assert_eq!(expected_result, result_string);
}
