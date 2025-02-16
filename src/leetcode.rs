use std::collections::HashMap;

fn _remove_occurrences(mut s: String, part: String) -> String {
    let part_len: usize = part.len();
    while let Some(idx) = s.find(&part) {
        s.replace_range(idx..idx + part_len, "");
    }
    s
}

fn _find_substring_index(main_string: &str, substring: &str) -> Option<usize> {
    if substring.is_empty() {
        return Some(0);
    }

    if main_string.is_empty() && !substring.is_empty() {
        return None;
    }

    for i in 0..=main_string.len() - substring.len() {
        let mut j = 0;
        while j < substring.len() && main_string.as_bytes()[i + j] == substring.as_bytes()[j] {
            j += 1;
        }
        if j == substring.len() {
            return Some(i);
        }
    }

    None
}

fn _maximum_sum(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = std::collections::HashMap::new();
    let mut ans = 0;
    // for num_str in nums.iter().map(|num| num.to_string()) {
    //     let num_digits_sum = num_str.chars().map(|c| c as i32).sum::<i32>();
        // if map.contains_key(&num_digits_sum) {
        //     ans = std::cmp::max(num_str.parse::<i32>().unwrap(), *map.get(&num_digits_sum).unwrap());
        // }
        // else {
        //     map.insert(num_digits_sum, num_str.parse::<i32>().unwrap());
        // }
    // }
    for num in nums {
        let digit_sum = num.to_string().chars().map(|c| (c as u8 - b'0') as i32).sum::<i32>();
        if map.contains_key(&digit_sum) {
            let map_val = *map.get(&digit_sum).unwrap();
            ans = std::cmp::max(ans, map_val + num);
            map.insert(digit_sum, map_val.max(num));
        }
        else {
            map.insert(digit_sum, num);
        }
    }
    ans
}

pub fn _min_operations(nums: Vec<i32>, k: i32) -> i32 {
        
        todo!()
}