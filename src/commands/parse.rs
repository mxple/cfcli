use crate::utils::*;
use scraper::{Html, Selector};
use futures::future::join_all;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleTest {
    input: String,
    expected: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCases {
    count: usize,
    tests: Vec<SingleTest>,
}

pub async fn parse(cp: &ContestOrProblem) {
    match cp {
        ContestOrProblem::Contest(contest) => {
            parse_contest(&contest).await;
        }
        ContestOrProblem::Problem(problem) => {
            parse_problem(&problem).await;
        }
    };
}

async fn parse_contest(c: &Contest) {
    // Query contest for how many problems
    let url = format!("https://codeforces.com/contest/{}", c.contest_id);
    let html = reqwest::get(&url)
        .await
        .unwrap_or_else(|_| panic!("Error fetching data from {}", &url))
        .text()
        .await
        .unwrap_or_else(|_| panic!("Error fetching data from {}", &url));

    let document = Html::parse_document(&html);

    // Parse problems using table entries
    let selector = Selector::parse("td.id a").unwrap();

    let mut futures = Vec::with_capacity(8);
    for problem in document.select(&selector) {
        // Extract the problem id from the href attribute value
        let pid = problem
            .value()
            .attr("href")
            .unwrap_or_default()
            .split('/')
            .last()
            .unwrap();

        let p = Problem {
            contest_id: c.contest_id,
            problem_id: pid.to_string(),
        };
        futures.push(async move { parse_problem(&p).await });
    }

    join_all(futures).await;
}

async fn parse_problem(p: &Problem) {
    let url = format!(
        "https://codeforces.com/contest/{}/problem/{}",
        p.contest_id, p.problem_id
    );
    let html = reqwest::get(&url)
        .await
        .unwrap_or_else(|_| panic!("Error fetching data from {}", &url))
        .text()
        .await
        .unwrap_or_else(|_| panic!("Error fetching data from {}", &url));

    let document = Html::parse_document(&html);
    let selector = Selector::parse(".sample-tests > .sample-test").unwrap();

    let mut test_cases = TestCases {
        count: 0,
        tests: Vec::new(),
    };

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

    // Turn parsed problem into folder/file
    let json_data = serde_json::to_string(&test_cases).unwrap();

    let cf_dir = "./";
    std::env::set_current_dir(cf_dir).expect("Unable to change to directory");

    fs::create_dir_all(p.contest_id.to_string()+&p.problem_id).expect("Unable to make directory");

    fs::write("tests.json", json_data).expect("Unable to write test cases to file");
}
