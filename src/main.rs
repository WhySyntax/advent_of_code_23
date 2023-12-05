use std::fs;
use std::cmp;

fn main() {
    let da_text = fs::read_to_string("input.txt").unwrap();
    println!("{}",_day_one(da_text));
}

fn _day_two(input:String) -> usize {
    let colors = ["red","green","blue"];
    let da_textier:Vec<&str> = input.split("\nGame ").map(|x| x.trim().split(": ").last().unwrap()).collect();
    let mut total = 0;

    for i in 0..da_textier.len() {
        let games:Vec<&str> = da_textier[i].split("; ").collect();
        let mut mins:[usize;3] = [0,0,0]; 
        for j in games.iter() {
            let aaahhhh:String = j.chars().filter(|&x| x != ',').collect();
            let game:Vec<&str> = aaahhhh.split(' ').collect();
            let mut exists:Option<usize>;
            for k in 0..3 {
                exists = game.iter().position(|&x| x == colors[k]);
                if exists.is_some() {
                    let amount = game[exists.unwrap() - 1].parse::<usize>().unwrap();
                    mins[k] = cmp::max(amount, mins[k]);
                }
            }
        }
        total += mins[0] * mins[1] * mins[2];
    }

    return total;
}

fn _day_one(input:String) -> u32 {
    let mut da_text= input;
    let mut da_textier:Vec<char> = da_text.chars().collect();
    let num_words = ["zero","one","two","three","four","five","six","seven","eight","nine"];
    for i in 0..10 {
        let mut loc = da_text.find(num_words[i]);
        while loc.is_some() {
            da_textier[loc.unwrap() + 1] = char::from_digit(i as u32, 10).unwrap();
            da_text = da_textier.clone().into_iter().collect();
            loc = da_text.find(num_words[i])
        }
    }
    let full:Vec<&str> = da_text.split("\n").collect();
    //split string into chars, just take the chars that are digits, multiply first by 10, add to last
    let mut total:u32 = 0;
    for input in full.iter() {
        let nums:Vec<u32> = input.chars()
            .filter(|&x| '0' <= x && '9' >= x)
            .map(|x| x.to_digit(10).unwrap()).collect();
        total += nums[0] * 10 + nums[nums.len()-1];
    }
    return total;
}