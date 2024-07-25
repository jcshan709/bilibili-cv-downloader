use std::{error::Error as StdError, fmt::Display, future::Future, io::{stdin, Write}, path::PathBuf, pin, time::Duration};
use regex::{self, Regex};
use reqwest::{get, Request};
use scraper::{Html, Selector};
use tokio::{io::AsyncWriteExt, task::{self, JoinHandle}};

enum LinkType {
    Link(String),
    CV(String),
    Unknown
}

fn check(input: &str) -> LinkType {
    let cv = match Regex::new(r"CV\d{1,}") {
        Err(_) => {
            return LinkType::Unknown;
        }
        Ok(r) => r,
    };
    let link = match Regex::new(r"https?://(www.)?bilibili.com/read/cv\d{1,}") {
        Err(_) => {
            return LinkType::Unknown;
        }
        Ok(r) => r,
    };
    if cv.is_match(input) {
        LinkType::CV(format!("https://bilibili.com/{}", input))
    }
    else if link.is_match(input) {
        LinkType::Link(input.to_string())
    }
    else {
        LinkType::Unknown
    }
}


async fn fetch(url: String) -> anyhow::Result<()> {
    let dir = match tokio::fs::read_dir(PathBuf::from("cv-images")).await {
        Ok(r) => r,
        Err(_) => {
            tokio::fs::create_dir(PathBuf::from("cv-images")).await?;
            tokio::fs::read_dir(PathBuf::from("cv-images")).await?
        }
    };
    let content = reqwest::get(url)
        .await?
        .text()
        .await?;
    let selector = Selector::parse("img[data-src]").unwrap();
    let doc = Html::parse_document(&content);
    let image_tags = doc.select(&selector);
    let mut tasks = Vec::new();

    for (idx, image_tag) in image_tags.enumerate() {
        let img_link = format!("https:{}", image_tag.attr("data-src").unwrap().split('@').next().unwrap());
        let ext = img_link.split('.').last().unwrap();
        let file_name = format!("cv-images/{}.{}", idx, ext);
        tasks.push(tokio::spawn((|img_link: String, file_name: String| async move {
            let mut file = tokio::fs::File::create(&file_name).await?;
            println!("[{}] 开始下载：{}", idx, img_link);
            file.write_all(&reqwest::get(img_link).await?.bytes().await?).await?;
            println!("[{}] 下载完成！", idx);
            anyhow::Result::<()>::Ok(())
        })(img_link, file_name.to_string())));
    }
    for task in tasks {
        match task.await {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Error while joining async tasks");
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    println!("输入链接或CV号：");
    stdin().read_line(&mut input)?;
    match check(&input) {
        LinkType::Link(url) => {
            fetch(url)
        }
        LinkType::CV(url) => {
            fetch(url)
        }
        _ => {
            panic!("链接错误！");
        }
    }.await?;
    println!("完成！");
    Ok(())
}
