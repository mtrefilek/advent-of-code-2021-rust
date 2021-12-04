use reqwest::{Url, ClientBuilder, header};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let day = 4;
    let step = 1;

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
        7 => four_one(resp).to_string(),
        8 => four_two(resp).to_string(),
        _ => "Not Implemented".to_string()
    };

    print!("{}", res);

    Ok(())
}

fn four_two(resp: String) -> i32 {
    return 0;
}

fn four_one(resp: String) -> i32 {
    let mut lines = resp.lines();
    let bingo_nums = lines.next().unwrap();

    let mut boards:  Vec<Vec<Vec<(i32, bool)>>> = Vec::new();

    let mut i: i32 = -1;
    for line in lines {
        if line.len() == 0 {
            let mut board: Vec<Vec<(i32, bool)>> = Vec::new();
            boards.push(board);
            i += 1;
        }
        else {
           let mut row: Vec<(i32, bool)> = Vec::new();
            for s in line.to_string().split_whitespace() {
                let num = s.parse::<i32>().unwrap();
                let mut tup = (num, false);
                row.push(tup);
            }
            boards[i as usize].push(row);
        }
    }

    let nums = bingo_nums.split(',');
    let mut wins = 0;
    let mut last_num = 0;
    let mut solved_boards: Vec<i32> = Vec::new();
    for num in nums {
        println!("{:?}", solved_boards);
        let num_i = num.parse::<i32>().unwrap();
        let mut board_num = 0;
        for mut board in boards.iter_mut() {
            let mut skip = false;
            for i in &solved_boards {
                if *i == board_num {
                    skip = true;
                }
            }
            if skip {
                board_num += 1;
                continue;
            }
            for mut row in board.iter_mut() {
                let mut index2 = 0;
                for index in row.iter_mut() {
                    if index.0 == num_i {
                        index.1 = true;
                    }
                    index2 += 1;
                }
            }
            board_num += 1;
        }

        board_num = 0;
        for board in &boards {
            let mut skip = false;
            for i in &solved_boards {
                if *i == board_num {
                    skip = true;
                }
            }
            if skip {
                board_num += 1;
                continue;
            }
            let mut solved = false;
            // Check Rows
            for row in board {
                let mut index = 0;
                for i in row {
                    if i.1 == false {
                        break;
                    } else if index == 4{
                        solved = true;
                    }
                    index += 1;
                }
            }
            for i in 0..5 {
                let mut index = 0;
                for row in board {
                    if row.get(i).unwrap().1 == false {
                        break;
                    } else if index == 4 {
                        solved = true;
                    }
                    index += 1;
                }
            }
            if solved {
                last_num = num_i;
                solved = false;
                solved_boards.push(board_num);
                let mut score = 0;
                for row in board {
                    for i in row {
                        if i.1 == false {
                            score += i.0;
                        }
                    }
                }
                wins = num_i * score;
                println!("{}", wins);
                println!("{}", num_i);
            }
            board_num += 1;
        }
    }
    return wins;
}

// Gross code but need to go to sleep
fn three_two(resp: String) -> i32 {
    let lines = resp.lines();
    let mut vec: Vec<(i32, i32)> = Vec::new();
    let mut vec_lines: Vec<String> = Vec::new();

    for line in lines {
        vec_lines.push(line.to_string());
        for (index, char) in line.chars().enumerate() {
            let i = char.to_digit(10).unwrap();
            if vec.get(index) == None {
                vec.push((0, 0))
            }
            if i == 0 {
                vec[index].0 += 1;
            } else if i == 1 {
                vec[index].1 += 1;
            }
        }
    }
    let mut vec_lines_2 = vec_lines.clone();

    let mut index = 0;
    for _tup2 in &vec {
        let mut tup = (0,0);

        for n in &vec_lines {
           if n.chars().nth(index).unwrap() == '0' {
               tup.0 += 1;
           } else {
               tup.1 += 1;
           }
        }

        let max: char;
        if tup.0 > tup.1 {
            max = '0';
        } else {
            max = '1';
        }
        let mut n = 0;
        loop {
            if vec_lines.len() == 1 {
                break;
            }
            if vec_lines.get(n as usize) == None {
                break;
            }
            if vec_lines[n as usize].chars().nth(index).unwrap() != max {
                vec_lines.remove(n as usize);
                n -= 1;
            }
            n+=1;
        }
        index += 1;
    }
    index = 0;
    for _tup2 in &vec {
        let mut tup = (0,0);

        for n in &vec_lines_2 {
            if n.chars().nth(index).unwrap() == '0' {
                tup.0 += 1;
            } else {
                tup.1 += 1;
            }
        }
        let min: char;
        if tup.0 > tup.1 {
            min = '1';
        } else {
            min = '0';
        }
        let mut n = 0;
        loop {
            if vec_lines_2.len() == 1 {
                break;
            }
            if vec_lines_2.get(n as usize) == None {
                break;
            }
            if vec_lines_2[n as usize].chars().nth(index).unwrap() != min {
                vec_lines_2.remove(n as usize);
                n -= 1;
            }
            n+=1;
        }
        index += 1;
    }


    let o_str = vec_lines[0].to_string();
    let c_str = vec_lines_2[0].to_string();
    let mut o = 0;
    let mut c = 0;
    let mut len = vec.len();
    let mut i = 0;
    for _tup in vec {
        len -= 1;
        let base: i32 = 2;
        o += o_str.chars().nth(i).unwrap().to_digit(10).unwrap() as i32 * base.pow(len as u32);
        c += c_str.chars().nth(i).unwrap().to_digit(10).unwrap() as i32 * base.pow(len as u32);

        i += 1;
    }
    return o*c;
}

fn three_one(resp: String) -> i32 {
    let lines = resp.lines();
    let mut vec: Vec<(i32, i32)> = Vec::new();

    for line in lines {
        for (index, char) in line.chars().enumerate() {
            let i = char.to_digit(10).unwrap();
            if vec.get(index) == None {
                vec.push((0, 0))
            }
            if i == 0 {
                vec[index].0 += 1;
            } else if i == 1 {
                vec[index].1 += 1;
            }
        }
    }

    println!("{:?}", vec);

    let mut g = 0;
    let mut e = 0;
    let mut len = vec.len();
    for tup in &vec {
        len -= 1;
        let base: i32 = 2;
        if tup.0 > tup.1 {
            e += base.pow(len as u32);
        } else {
            g += base.pow(len as u32);
        }
    }
    return g*e;
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