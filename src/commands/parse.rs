use crate::utils::*;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct SingleTest {
    input: String,
    expected: String,
}

#[derive(Debug)]
pub struct TestCases {
    count: usize,
    tests: Vec<SingleTest>,
}

pub async fn parse(p: &Problem) -> Result<TestCases, reqwest::Error> {
    let html = reqwest::get(format!(
        "https://codeforces.com/contest/{}/problem/{}",
        p.contest_id, p.problem_id
    ))
    .await?
    .text()
    .await?;

    let mut test_cases = TestCases {
        count: 0,
        tests: Vec::new(),
    };

    let document = Html::parse_document(&html);
    let selector = Selector::parse(".sample-tests > .sample-test").unwrap();

    for sample_test in document.select(&selector) {
        // Select input and output divisions within the current sample-test division
        let isel = Selector::parse(".input > pre").unwrap();
        let osel = Selector::parse(".output > pre").unwrap();
        let inputs = sample_test.select(&isel);
        let outputs = sample_test.select(&osel);

        // Iterate over input and output divisions
        for (input, output) in inputs.zip(outputs) {
            test_cases.count += 1;
            test_cases.tests.push(SingleTest {
                input: input.text().collect::<Vec<_>>().join("\n"),
                expected: output.text().collect::<Vec<_>>().join("\n"),
            });
        }
    }
    Ok(test_cases)
}
