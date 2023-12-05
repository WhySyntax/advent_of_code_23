use std::fs;
use std::cmp;

fn main() {
    let da_text = fs::read_to_string("input.txt").unwrap();
    println!("{}",day_three(da_text));
}

fn day_three (input:String) -> usize {
    let cleaned:String = input.chars()
        .filter(|&x| x != '\n') // makes entire input into a single string for me to clean up later
        .map(|x| {
            if (('0' >= x) || ('9' <= x)) && (x != '.') {
                'a' // makes special characters more easily identifiable
            } else {x}
        }).collect();
    let mut total = 0;
    let mut nums:Vec<[usize;2]> = Vec::new();
    return total;
}

fn _day_two(input:String) -> usize {
    let colors = ["red","green","blue"];
    let da_textier:Vec<&str> = input.split("\nGame ").map(|x| x.trim().split(": ").last().unwrap()).collect(); // separates each game's input
    let mut total = 0;

    for i in 0..da_textier.len() {
        let games:Vec<&str> = da_textier[i].split("; ").collect(); // removes game # because I can use the loop index for that
        let mut mins:[usize;3] = [0,0,0]; 
        for j in games.iter() {
            let aaahhhh:String = j.chars().filter(|&x| x != ',').collect(); // removes the commas so I don't have to deal with them when parsing nums
            let game:Vec<&str> = aaahhhh.split(' ').collect(); // separates nums from colors
            let mut exists:Option<usize>;
            for k in 0..3 {
                exists = game.iter().position(|&x| x == colors[k]); // checks if a color is used and puts the amount in mins if needed
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
    for i in 0..10 { // makes the 2nd letter of each digit word into the digit itself
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