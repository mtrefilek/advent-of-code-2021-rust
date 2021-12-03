use reqwest::{Url, ClientBuilder, header};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let day = 2;
    let step = 2;

    execute(day, step).await?;

    Ok(())
}

async fn get_input(day: i32) -> Result<String, Box<dyn std::error::Error>> {
    let session_token = env::var("AOC_SESSION")?;
    let str = format!("https://adventofcode.com/2021/day/{}/input", day);
    let url = str.parse::<Url>().unwrap();
    let mut headers = header::HeaderMap::new();
    let token = format!("session={}", session_token).to_string();
    headers.insert("Cookie", header::HeaderValue::from_str(&*token)?);
    let client = ClientBuilder::new().default_headers(headers).build()?;

    let resp = client.get(url).send().await?
        .text()
        .await?;
    Ok(resp)
}

async fn execute(day: i32, step: i32) -> Result<(), Box<dyn std::error::Error>> {
    let resp = get_input(day).await?;

    let res = match day * 2 - 2 + step {
        1 => {
            one_one(resp).to_string()
        },
        2 => {
            one_two(resp).to_string()
        },
        3 => {
            two_one(resp).to_string()
        },
        4 => {
            two_two(resp).to_string()
        },
        5 => {
            three_one(resp).to_string()
        },
        6 => three_two(resp).to_string(),
        _ => "Not Implemented".to_string()
    };

    print!("{}", res);

    Ok(())
}

fn three_two(resp: String) -> i32 {

    return 0;
}

fn three_one(resp: String) -> i32 {

    return 0;
}

fn two_two(resp: String) -> i32 {
    let lines = resp.lines();
    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;

    for line in lines {
        let mut l = line.split_whitespace();
        let dir = l.next().unwrap();
        let value = l.next().unwrap().parse::<i32>().unwrap();
        if dir == "forward" {
            h += value;
            d += aim * value;
        } else if dir == "up" {
            aim -= value
        } else if dir == "down" {
            aim += value
        }
    }

    return h*d;
}

fn two_one(resp: String) -> i32 {
    let lines = resp.lines();
    let mut h = 0;
    let mut d = 0;

    for line in lines {
        let mut l = line.split_whitespace();
        let dir = l.next().unwrap();
        let value = l.next().unwrap().parse::<i32>().unwrap();
        if dir == "forward" {
            h += value;
        } else if dir == "up" {
            d -= value
        } else if dir == "down" {
            d += value
        }
    }

    return h*d;
}

fn one_two(resp: String) -> i32 {
    let lines = resp.lines();
    let mut v: Vec<(i32, i32)> = Vec::new();

    let mut count = 0;
    for (index, line) in lines.enumerate() {
        let num = line.parse::<i32>().unwrap();
        if v.get(index) == None {
            v.push((num, 0));
        } else {
            v[index].0 += num;
            v[index].1 += 1;
        }
        if v.get(index + 1) == None {
            v.push((num, 0));
        } else {
            v[index+1].0 += num;
            v[index+1].1 += 1;
        }
        if v.get(index + 2) == None {
            v.push((num, 0));
        } else {
            v[index+2].0 += num;
            v[index+2].1 += 1;
        }
    }

    let mut last_window = v[2].0;
    for tup in &v[3..] {
        if tup.1 < 2 {
            break;
        }
        if tup.0 > last_window {
            count += 1;
        }
        last_window = tup.0;
    }

    return count;
}

fn one_one(resp: String) -> i32 {
    let mut lines = resp.lines();
    let mut count = 0;
    let mut pastnum = lines.next().unwrap().parse::<i32>().unwrap();
    for (_index, line) in lines.enumerate() {
        let num = line.parse::<i32>().unwrap();
        if num > pastnum {
            count += 1;
        }
        pastnum = num;
    }
    return count;
}