use scraper::Html;
use colored::Colorize;
use regex::Regex;
use rpassword::read_password;
use crate::util;

fn get_ftaa() -> String {
    util::random_string(18)
}

fn get_bfaa() -> String {
    "f1b3f18c715565b589b7823cda7448ce".to_string()
}

fn get_csrf(body: &String) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(&body);
    let selector = scraper::Selector::parse("span.csrf-token").unwrap();

    let csrf_token = document.select(&selector)
        .next()
        .ok_or("No csrf token found")?
        .value()
        .attr("data-csrf")
        .ok_or("No csrf token found")?
        .to_string();

    Ok(csrf_token)
}

fn get_credentials() -> Result<(String, String), Box<dyn std::error::Error>> {
    println!("{}", "Enter your Codeforces username:".blue().bold());
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;
    username = username.trim().to_string();

    println!("{}", "Enter your Codeforces password:".blue().bold());
    let password = read_password()?;
    Ok((username, password))
}

pub async fn login() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build().unwrap();
    let body = client.get("https://codeforces.com/")
        .send()
        .await?
        .text()
        .await?;

    let ftta = get_ftaa();
    let bfaa = get_bfaa();
    let csrf = get_csrf(&body)?;
    let (username, password) = get_credentials()?;

    let form = [
        ("action", "enter"),
        ("ftaa", &ftta),
        ("bfaa", &bfaa),
        ("handleOrEmail", &username),
        ("password", &password),
        ("csrf_token", &csrf),
        ("_tta", "182"),
        ("remember", "on"),
    ];

    let response = client.post("https://codeforces.com/enter?back=/")
        .form(&form)
        .send()
        .await?;

    let post_login_body = response.text().await?;
    let Ok(handle) = get_handle(&post_login_body) else {
        println!("{}", "Login failed, try again.".red().bold());
        return Err(Box::from("Login failed"));
    };

    println!("{} {}", "Successfully logged in with handle".green(), handle.green().bold());

    Ok(())
}

fn get_handle(body: &String) -> Result<String, Box<dyn std::error::Error>> {
    let handle_regex = Regex::new(r#"handle = "([\s\S]+?)""#)?;
    let handle = handle_regex.captures(body)
        .ok_or("No handle found")?
        .get(1)
        .ok_or("No handle found")?
        .as_str()
        .to_string();

    Ok(handle)
}

