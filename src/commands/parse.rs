use crate::utils::*;
use futures::future::join_all;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{fs, io::Read, path::PathBuf, process::Command};

use self::state::Config;

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

pub async fn parse(cp: &ContestOrProblem, config: &Config) {
    match cp {
        ContestOrProblem::Contest(contest) => {
            parse_contest(&contest, config).await;
        }
        ContestOrProblem::Problem(problem) => {
            parse_problem(&problem, config).await;
        }
    };
}

async fn parse_contest(c: &Contest, config: &Config) {
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
        futures.push(async move { parse_problem(&p, config).await });
    }

    join_all(futures).await;
}

async fn parse_problem(p: &Problem, config: &Config) {
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

    // Write to cache
    let mut cache_path = PathBuf::from(&config.cf_dir);

    cache_path.push(".cfcli");
    cache_path.push(p.contest_id.to_string() + &p.problem_id);

    fs::create_dir_all(&cache_path).expect("Unable to make directory");

    cache_path.push("tests.json");
    fs::write(cache_path, json_data).expect("Unable to write test cases to file");

    // Create workspace files
    let mut workspace_path = PathBuf::from(&config.cf_dir);

    // Replace placeholders with actual values
    workspace_path.push(
        &config
            .workspace_dir
            .replace("{%contest_id%}", &p.contest_id.to_string())
            .replace("{%problem_id%}", &p.problem_id),
    );

    fs::create_dir_all(&workspace_path).expect("Unable to make directory");
    std::env::set_current_dir(&workspace_path).expect("Failed to change to workspace directory");

    Command::new("bash")
        .arg("-c")
        .arg(&config.workspace_creation_cmd)
        .output()
        .expect("Failed to execute workspace creation command");

    // Pull template file
    let mut template_path = PathBuf::from(&config.cf_dir);
    template_path.push(".cfcli/templates/");
    // hard-coded cpp for now
    template_path.push("template.cpp");

    let mut template_file = fs::File::open(&template_path).expect("Could not open code template");
    let mut template_buffer = String::new();
    template_file
        .read_to_string(&mut template_buffer)
        .expect("Error reading template file");

    // replace time date stuff idk
    // template_buffer.replace();

    let mut solution_path = PathBuf::from(workspace_path);
    let mut solution_filename = config
            .solution_filename
            .clone()
            .replace("{%contest_id%}", &p.contest_id.to_string())
            .replace("{%problem_id%}", &p.problem_id);
    solution_filename.push_str(".cpp");
    solution_path.push(solution_filename);
    fs::write(solution_path, template_buffer).expect("Unable to write file");
}
