use reqwest::{Url, ClientBuilder, header};
use std::env;
use std::collections::{HashMap, VecDeque};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let day = 9;
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
        7 => four_one(resp).to_string(),
        8 => four_two(resp).to_string(),
        9 => five_one(resp).to_string(),
        10 => five_two(resp).to_string(),
        11 => six_one(resp).to_string(),
        12 => six_two(resp).to_string(),
        13 => seven_one(resp).to_string(),
        14 => seven_two(resp).to_string(),
        15 => eight_one(resp).to_string(),
        16 => eight_two(resp).to_string(),
        17 => nine_one(resp).to_string(),
        18 => nine_two(resp).to_string(),
        _ => "Not Implemented".to_string()
    };

    print!("{}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{five_two, six_one, six_two, seven_one, seven_two, eight_one, eight_two, nine_one, nine_two};
    use std::fs;

    #[test]
    fn five_two_works() {
        let str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";
        assert_eq!(five_two(str.parse().unwrap()), 12)
    }

    #[test]
    fn six_one_works() {
        let str = "3,4,3,1,2";
        assert_eq!(six_one(str.parse().unwrap()), 5934)
    }

    #[test]
    fn six_two_works() {
        let str = "3,4,3,1,2";
        assert_eq!(six_two(str.parse().unwrap()), 26984457539)
    }

    #[test]
    fn seven_one_works() {
        let str = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(seven_one(str.parse().unwrap()), 37);
    }

    #[test]
    fn seven_two_works() {
        let str = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(seven_two(str.parse().unwrap()), 168);
    }

    #[test]
    fn eight_one_works() {
        let str = fs::read_to_string("static/eight.in");
        assert_eq!(eight_one(str.unwrap()), 26);
    }

    #[test]
    fn eight_two_works() {
        let str = fs::read_to_string("static/eight.in");
        assert_eq!(eight_two(str.unwrap()), 61229);
    }

    #[test]
    fn nine_one_works() {
        let str = fs::read_to_string("static/nine.in");
        assert_eq!(nine_one(str.unwrap()), 15);
    }

    #[test]
    fn nine_two_works() {
        let str = fs::read_to_string("static/nine.in");
        assert_eq!(nine_two(str.unwrap()), 1134);
    }
}

fn nine_two(resp:String) -> i32 {
    let mut map:Vec<Vec<i32>> = Vec::new();
    for line in resp.lines() {
        let mut v:Vec<i32> = Vec::new();
        for char in line.chars() {
            let z = char.to_digit(10).unwrap();
            v.push(z as i32);
        }
        map.push(v);
    }

    let mut basin_memoize:HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut basin_count: HashMap<(usize, usize), i32> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            match basin_memoize.get(&(x, y)) {
                Some(i) => {
                    let sum = basin_count.entry(*i).or_default();
                    *sum += height;
                    basin_memoize.insert((x, y), *i);
                    continue
                },
                None => {}
            }

            match map.get(y) {
                Some(r) => {
                    match r.get(x) {
                        Some(h) => {
                            if *h == 9 {
                                continue
                            }
                        },
                        None => {

                        }
                    }
                },
                None => {}
            }

            let minima = gradient_descent((x, y), &map);
            basin_memoize.insert((x, y), minima);
            let sum = basin_count.entry(minima).or_default();
            *sum += 1;
        }
    }
    println!("{:?}", basin_count);
    let mut vec:Vec<&i32> = basin_count.values().collect();
    let mut sum = 1;
    vec.sort();
    for i in 1 .. 4 {
        sum *= vec[vec.len() - i];
    }
    println!("{:?}", vec);
    sum
}

fn gradient_descent(index:(usize, usize), map:&Vec<Vec<i32>>) -> (usize, usize) {
    if local_minima(index, &map) {
        return index;
    } else {
        return gradient_descent(steepest_gradient(index, &map), &map);
    }
}

fn local_minima(index:(usize, usize), map:&Vec<Vec<i32>>) -> bool {
    let height =  match map.get(index.1) {
        Some(r) => {
            match r.get(index.0) {
                Some(h) => {
                    h
                },
                None => {
                   & 0
                }
            }
        },
        None => {&0}
    };
    let x = index.0;
    let y = index.1;
    match map.get(((y as i32) - 1) as usize) {
        Some(r) => {
            match r.get(x) {
                Some(h) => {
                    if h <= height {
                        return false
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    match map.get(((y as i32)+1) as usize) {
        Some(r) => {
            match r.get(x) {
                Some(h) => {
                    if h <= height {
                        return false
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    match map.get(y) {
        Some(r) => {
            match r.get(((x as i32) -1) as usize) {
                Some(h) => {
                    if h <= height {
                        return false
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    match map.get(y) {
        Some(r) => {
            match r.get(((x as i32) +1) as usize) {
                Some(h) => {
                    if h <= height {
                        return false
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    true
}

fn steepest_gradient(index:(usize, usize), map:&Vec<Vec<i32>>) -> (usize, usize) {
    let height =  match map.get(index.1) {
        Some(r) => {
            match r.get(index.0) {
                Some(h) => {
                    h
                },
                None => {
                    &0
                }
            }
        },
        None => {&0}
    };
    let x = index.0;
    let y = index.1;
    let mut max_grad = 0;
    let mut max_grad_i = (x, y);
    match map.get(((y as i32) - 1) as usize) {
        Some(r) => {
            match r.get(x) {
                Some(h) => {
                    if h <= height && height - h > max_grad {
                        max_grad = height - h;
                        max_grad_i = (x, y-1);
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    match map.get(((y as i32)+1) as usize) {
        Some(r) => {
            match r.get(x) {
                Some(h) => {
                    if h <= height && height - h > max_grad {
                        max_grad = height - h;
                        max_grad_i = (x, y+1);
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    match map.get(y) {
        Some(r) => {
            match r.get(((x as i32) -1) as usize) {
                Some(h) => {
                    if h <= height && height - h > max_grad {
                        max_grad = height - h;
                        max_grad_i = (x-1, y);
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    match map.get(y) {
        Some(r) => {
            match r.get(((x as i32) +1) as usize) {
                Some(h) => {
                    if h <= height && height - h > max_grad {
                        max_grad = height - h;
                        max_grad_i = (x+1, y);
                    }
                },
                None => {}
            }
        },
        None => {}
    }
    max_grad_i
}

fn nine_one(resp:String) -> i32 {
    let mut map:Vec<Vec<i32>> = Vec::new();
    for line in resp.lines() {
        let mut v:Vec<i32> = Vec::new();
        for char in line.chars() {
            let z = char.to_digit(10).unwrap();
            v.push(z as i32);
        }
        map.push(v);
    }
    let mut calc = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {

            match map.get(((y as i32) - 1) as usize) {
                Some(r) => {
                    match r.get(x) {
                        Some(h) => {
                            if h <= height {
                                continue
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
            match map.get(((y as i32)+1) as usize) {
                Some(r) => {
                    match r.get(x) {
                        Some(h) => {
                            if h <= height {
                                continue
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
            match map.get(y) {
                Some(r) => {
                    match r.get(((x as i32) -1) as usize) {
                        Some(h) => {
                            if h <= height {
                                continue
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
            match map.get(y) {
                Some(r) => {
                    match r.get(((x as i32) +1) as usize) {
                        Some(h) => {
                            if h <= height {
                                continue
                            }
                        },
                        None => {}
                    }
                },
                None => {}
            }
            calc += height + 1;
        }
    }
    calc
}

fn eight_two(resp:String) -> i32 {
    // 2: 1; 3: 7; 4: 4; 5: 2,3,5 6: 0, 6, 9 7: 0
    // We know, 1, 7, 4, and 8
    // item in seven not in one is on the top; two items from 1 on right side
    // letter that is in 7 But not in one of the 3 6-count strings is the top right
    // letter in one that is not top right is bottom right
    // for all 5 letters, item that is not in 7 but is the same in all is either middle or bottom
    // Of the above two, letter that is in all of 6 count is bottom and other one is middle
    // We know 4, so the unknown left side shared with 4 is top left
    // finally we get top right

    // 0 2  5
    //   3
    // 1 4  6
    // TEST: ab right side in 1 so d is top
    // a top right
    // b bottom right
    // c and f are middles
    // c is bottom
    // f middle
    // e top right
    // g bottom right
    let mut ans = 0;
    for line in resp.lines() {
        let v: Vec<&str> = line.split(" | ").collect();
        let mut lengths:HashMap<usize, Vec<&str>> = HashMap::new();
        let l: Vec<&str> = v[0].split(" ").collect();
        for s in l {
            let lengths_i = lengths.entry(s.len()).or_insert(Vec::new());
            lengths_i.push(s);
        }
        let r: Vec<&str> = v[1].split(" ").collect();

        let mut key:HashMap<i8, char> = HashMap::new();

        // get top
        let seven = lengths.get(&3).unwrap()[0];
        let one = lengths.get(&2).unwrap()[0];

        for char in seven.chars() {
            match one.find(char) {
                Some(i) => {},
                None => {key.insert(2, char);}
            }
        }

        for char in seven.chars() {
            for s in lengths.get(&6).unwrap() {
                match s.find(char) {
                    Some(i) => {},
                    None => {
                        key.insert(5, char);
                        for char2 in one.chars() {
                            if char != char2 {
                                key.insert(6, char2);
                            }
                        }
                    }
                }
            }
        }

        let mut chars_tmp:HashMap<char, i8> = HashMap::new();
        for s in lengths.get(&5).unwrap() {
            for char in s.chars() {
                let cnt = chars_tmp.entry(char).or_insert(0);
                *cnt += 1;
            }
        }

        for (char, i) in chars_tmp {
            if i != 3 {
                continue
            }
            match seven.find(char) {
                Some(i) => {

                },
                None => {
                    let mut found = true;
                    for s in lengths.get(&6).unwrap() {
                        match s.find(char) {
                            Some(i) => {},
                            None => {found = false}
                        }
                    }
                    if found {
                        key.insert(4, char);
                    } else {
                        key.insert(3, char);
                    }
                }
            }
        }
        let four = lengths.get(&4).unwrap()[0];
        let mut found:char = 'z';
        for char in four.chars() {
            let mut in_str = false;
            for (i, char2) in &key {
                if char == *char2 {
                    in_str = true;
                }
            }
            if !in_str {
                key.insert(0, char);
            }
        }
        let eight = lengths.get(&7).unwrap()[0];
        for char in eight.chars() {
            let mut in_str = false;
            for (i, char2) in &key {
                if char == *char2 {
                    in_str = true;
                }
            }
            if !in_str {
                key.insert(1, char);
            }
        }

        let mut k_inv: HashMap<char, i8> = HashMap::new();
        for (i, char) in &key {
            k_inv.insert(*char, *i);
        }
        let mut count = 1000;
        let mut num = 0;

        for s in r {
            let mut dig = 0;
            if s.len() == 2 {
                dig = 1;
            } else if s.len() == 3 {
                dig = 7;
            } else if s.len() == 4 {
                dig = 4;
            } else if s.len() == 7 {
                dig = 8;
            } else {
                let mut vec:Vec<i8> = Vec::new();
                for char in s.chars() {
                    vec.push(*k_inv.get(&char).unwrap());
                }
                vec.sort();
                if vec == [0, 1, 2, 4, 5, 6] {
                    dig = 0;
                } else if vec == [1, 2, 3, 4, 5] {
                    dig = 2;
                } else if vec == [2, 3, 4, 5, 6] {
                    dig = 3;
                } else if vec == [0, 2, 3, 4, 6] {
                    dig = 5;
                } else if vec == [0, 1, 2, 3, 4, 6] {
                    dig = 6;
                } else if vec == [0, 2, 3, 4, 5, 6] {
                    dig = 9;
                }
            }
            num += dig * count;
            count = count / 10;
        }
        ans += num;
    }
    ans
}

fn eight_one(resp:String) -> i64 {
    let mut count = 0;
    for line in resp.lines() {
        let v:Vec<&str> = line.split(" | ").collect();
        let l:Vec<&str> = v[0].split(" ").collect();
        let r:Vec<&str> = v[1].split(" ").collect();

        for s in r {
            if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 {
                count += 1;
            }
        }
    }
    count
}

fn seven_two(mut resp:String) -> i64 {
    // Algorithm for this one is: (|y-x|(|y-x|+1))/2
    // Where y is the starting position. This creates an exponential function and we have to find the minima. We will perform a binary search
    //Looking for a crate to perform a dertivative function
    let len = resp.trim_end_matches(&['\r', '\n'][..]).len();
    resp.truncate(len);
    let b: Vec<i64> = resp.split(",").map(|str| {str.parse::<i64>().unwrap()}).collect();

    let mut high = *b.iter().max().unwrap();
    let mut low = 0;
    loop {
        let x = (high-low)/2 + low;
        let tup = (dist(&b, x-1), dist(&b, x), dist(&b, x+1));
        if tup.1 < tup.0 && tup.1 < tup.2 {
            return tup.1
        } else if tup.1 < tup.0 {
            low = x+1;
        } else {
            high = x-1;
        }
    }
}

fn dist(v:&Vec<i64>, x:i64) -> i64 {
    v.into_iter().map(|y| ((y-x).abs() * ((y-x).abs() + 1) / 2)).sum()
}

fn seven_one(mut resp:String) -> i64 {
    let len = resp.trim_end_matches(&['\r', '\n'][..]).len();
    resp.truncate(len);
    let mut b: Vec<i64> = resp.split(",").map(|str| {str.parse::<i64>().unwrap()}).collect();
    b.sort();
    let mut queue:VecDeque<i64> = VecDeque::from(b.clone());
    let mut until = if b.len() % 2 == 0 { 2 } else { 1 };
    while queue.len() > until {
        queue.pop_front();
        queue.pop_back();
    }
    let mut median = 0;
    for a in queue.iter() {
        median += *a;
    }
    median = median / (until as i64);

    let mut count = 0;
    for i in b {
        count += (i - median).abs();
    }

    count
}

fn six_two(resp: String) -> i128 {
    let b = resp.split(",");
    let mut vec:Vec<i128> = Vec::new();
    let mut queue:VecDeque<i128> = VecDeque::new();

    for i in 0 .. 7 {
        vec.push(0);
    }
    for i in 0 .. 9 {
        queue.push_back(0);
    }
    for s in b {
        let str = s.parse::<usize>();
        match str {
            Ok(i) => {
                match vec.get_mut(i) {
                    Some(j) => {
                        *j += 1;
                    }
                    None => {
                        vec[i] = 1;
                    }
                }
            },
            Err(err) => {println!("Error: {}", s); vec[3] += 1;}
        }
    }
    println!("{:?}", vec);

    for i in 0 .. 256 {
        let mod_i = i % 7;
        let x = vec[mod_i];
        if let Some(y) = vec.get_mut(mod_i) {
            *y += queue.pop_front().unwrap();
            queue.push_back(*y);
        }
    }
    let mut count:i128 = 0;
    for i in queue.iter() {
        count += *i;
    }
    for v in vec {
        count += v;
    }
    count
}

fn six_one(resp: String) -> i128 {
    let b = resp.split(",");
    let mut vec:Vec<i128> = Vec::new();
    let mut queue:VecDeque<i128> = VecDeque::new();

    for i in 0 .. 7 {
        vec.push(0);
    }
    for i in 0 .. 9 {
        queue.push_back(0);
    }
    for s in b {
        let str = s.parse::<usize>();
        match str {
            Ok(i) => {
                match vec.get_mut(i) {
                    Some(j) => {
                        *j += 1;
                    }
                    None => {
                        vec[i] = 1;
                    }
                }
            },
            Err(err) => {println!("Error: {}", s); vec[3] += 1;}
        }
    }

    for i in 0 .. 80 {
        let mod_i = i % 7;
        let x = vec[mod_i];
        if let Some(y) = vec.get_mut(mod_i) {
            *y += queue.pop_front().unwrap();
            queue.push_back(*y);
        }
    }
    let mut count:i128 = 0;
    for i in queue.iter() {
        count += *i;
    }
    for v in vec {
        count += v;
    }
    count
}

fn five_two(resp: String) -> i32 {
    let mut lines = resp.lines();
    let mut count = 0;
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        let mut s = line.split_whitespace();

        let mut begin = s.next().unwrap().split(",");
        s.next();
        let mut end = s.next().unwrap().split(",");

        let b_x = begin.next().unwrap().parse::<i32>().unwrap();
        let b_y = begin.next().unwrap().parse::<i32>().unwrap();
        let e_x = end.next().unwrap().parse::<i32>().unwrap();
        let e_y = end.next().unwrap().parse::<i32>().unwrap();

        let start_x = if b_x <= e_x { b_x } else { e_x };
        let end_x = if b_x <= e_x { e_x } else { b_x };

        let start_y = if b_x <= e_x { b_y } else { e_y };
        let end_y = if b_x <= e_x { e_y } else { b_y };
        println!("{}, {}, {}, {}", start_x, start_y, end_x, end_y);

        let mut grad = (0, 0);
        let delta_x = if start_x == end_x { 0 } else { 1 };
        let delta_y = if start_y < end_y { 1 } else if start_y > end_y { -1 } else { 0 };

        let mut i = start_x;
        let mut j = start_y;

        while i != end_x || j != end_y {
            if !map.contains_key(&(i, j)) {
                map.insert((i, j), 1);
            } else {
                let val = map.entry((i,j)).or_default();
                if *val == 1 {
                    count += 1;
                }
                *val += 1;
            }
            println!("{}, {}: {}", i, j, map.get(&(i,j)).unwrap());
            grad.1 = grad.1 + delta_y;
            grad.0 = grad.0 + delta_x;
            j = j + delta_y;
            i = i + delta_x;
        }
        if !map.contains_key(&(i, j)) {
            map.insert((i, j), 1);
        } else {
            let val = map.entry((i,j)).or_default();
            if *val == 1 {
                count += 1;
            }
            *val += 1;
        }
        println!("{}, {}: {}", i, j, map.get(&(i,j)).unwrap());
    }
    count
}

fn five_one(resp: String) -> i32 {
    let mut lines = resp.lines();
    let mut count = 0;
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        let mut s = line.split_whitespace();
        let mut begin = s.next().unwrap().split(",");
        s.next();
        let mut end = s.next().unwrap().split(",");

        let b_x = begin.next().unwrap().parse::<i32>().unwrap();
        let b_y = begin.next().unwrap().parse::<i32>().unwrap();
        let e_x = end.next().unwrap().parse::<i32>().unwrap();
        let e_y = end.next().unwrap().parse::<i32>().unwrap();

        if (b_x != e_x) && (b_y != e_y) {
            continue;
        }

        let start_x = if b_x <= e_x { b_x } else { e_x };
        let end_x = if b_x <= e_x { e_x } else { b_x };

        let start_y = if b_y <= e_y { b_y } else { e_y };
        let end_y = if b_y <= e_y { e_y } else { b_y };
        println!("{}, {}, {}, {}", start_x, start_y, end_x, end_y);

        for i in start_x .. end_x+1 {
            for j in start_y .. end_y+1 {
                if !map.contains_key(&(i, j)) {
                    map.insert((i, j), 1);
                } else {
                    let val = map.entry((i,j)).or_default();
                    if *val == 1 {
                        count += 1;
                    }
                    *val += 1;
                }
            }
        }
    }

    count
}

fn four_two(resp: String) -> i32 {
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
                if wins == 0 {
                    wins = num_i * score;
                }
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