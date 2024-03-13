use rand::Rng;

const ALPHA_NUMERIC: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

pub fn random_string(n: usize) -> String {
    let mut rng = rand::thread_rng();
    let s: String = (0..n)
        .map(|_| {
            let i = rng.gen_range(0..ALPHA_NUMERIC.len());
            ALPHA_NUMERIC.chars().nth(i).unwrap()
        })
        .collect();
    s
}