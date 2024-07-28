use colored::Colorize;
use std::time::Duration;
use thirtyfour::{error::WebDriverResult, By, DesiredCapabilities, WebDriver, WebElement};
use tokio::time::sleep;

#[tokio::main]
// fn main() {
async fn main() -> WebDriverResult<()> {
    let driver = setup().await.expect("Failed to setup driver");

    let all_rows = driver
        .find_all(By::Css(
            "body center table tbody tr:nth-child(3) td table tbody tr",
        ))
        .await?;

    let rows = get_rows(&all_rows).await;
    let info_rows = get_info_rows(&all_rows).await;

    let _ = print_articles(rows, info_rows).await;

    Ok(())
}

async fn get_rows(all_rows: &Vec<WebElement>) -> Vec<WebElement> {
    let mut rows: Vec<WebElement> = Vec::new();
    for row in all_rows {
        let class_name = row.class_name().await.unwrap();
        if let Some(class_name) = class_name {
            if class_name == "athing" {
                rows.push(row.to_owned())
            }
        }
    }

    rows
}

async fn get_info_rows(all_rows: &Vec<WebElement>) -> Vec<WebElement> {
    let mut rows: Vec<WebElement> = Vec::new();
    for row in all_rows {
        let class_name = row.class_name().await.unwrap();
        if class_name.is_none() {
            rows.push(row.to_owned())
        }
    }

    rows
}

async fn print_articles(rows: Vec<WebElement>, info_rows: Vec<WebElement>) -> Result<(), String> {
    let width = 100;
    for (i, row) in rows.iter().enumerate() {
        if i < info_rows.len() {
            let title = row
                .find(By::Css(".title span a"))
                .await
                .expect("Failed to read title element")
                .text()
                .await
                .expect("Failed to extract text");

            let url: String = row
                .find(By::Css(".title .titleline a"))
                .await
                .expect("Failed to read points element")
                .attr("href")
                .await
                .expect("Failed to read href")
                .unwrap();

            let author = info_rows[i]
                .find(By::Css(".subtext .subline a"))
                .await
                .expect("Failed to read author element")
                .text()
                .await
                .expect("Failed to read auther text");

            let age = info_rows[i]
                .find(By::Css(".subtext .subline .age a"))
                .await
                .expect("Failed to read age element")
                .text()
                .await
                .expect("Failed to read age text");

            let points: String = info_rows[i]
                .find(By::Css(".subtext span span"))
                .await
                .expect("Failed to read points element")
                .text()
                .await
                .expect("Failed to read points text");

            println!("{}", "_".repeat(width).bright_white());
            println!(
                "{}. {}",
                (i + 1).to_string().bright_white(),
                points.bright_blue()
            );
            println!("{}", title.bright_green());
            println!(
                "{} {} {}",
                "By".bright_yellow(),
                author.bright_yellow(),
                age.bright_yellow()
            );
            println!("{}", url.cyan());
            println!("{}\n", "_".repeat(width).bright_white());
        }
    }
    Ok(())
}

async fn setup() -> Result<WebDriver, String> {
    let caps = DesiredCapabilities::chrome();
    println!("Loading web page...");

    let driver = WebDriver::new("http://localhost:9515", caps)
        .await
        .expect("Failed to connect to chromedriver");
    println!("Loading web page...");

    driver
        .goto("https://news.ycombinator.com/")
        .await
        .expect("Failed to go to url");
    sleep(Duration::from_millis(1500)).await;

    println!("Loaded web page!");

    Ok(driver)
}
