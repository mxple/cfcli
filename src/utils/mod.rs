use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum ContestOrProblem {
    Contest(Contest),
    Problem(Problem),
}

#[derive(Debug, Clone)]
pub struct Problem {
    pub contest_id: u32,
    pub problem_id: String,
}

#[derive(Debug, Clone)]
pub struct Contest {
    pub contest_id: u32,
}

impl FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_end = s
            .chars()
            .position(|c| c.is_alphabetic())
            .ok_or(String::new())?;
        let (contest, problem) = s.split_at(num_end);

        let contest_fromstr = contest
            .parse::<u32>()
            .map_err(|_| String::from("Error parsing problem"))?;

        Ok(Problem {
            contest_id: contest_fromstr,
            problem_id: problem.to_string(),
        })
    }
}

impl FromStr for Contest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().position(|c| c.is_alphabetic()).is_some() {
            return Err(String::from("Error parsing contest: found letter"));
        }

        Ok(Contest {
            contest_id: s
                .parse::<u32>()
                .map_err(|_| String::from("Error parsing contest"))?,
        })
    }
}

impl FromStr for ContestOrProblem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_end = s.chars().position(|c| c.is_alphabetic());
        if num_end.is_none() {
            return Ok(ContestOrProblem::Contest(Contest::from_str(s)?));
        }

        Ok(ContestOrProblem::Problem(Problem::from_str(s)?))
    }
}
