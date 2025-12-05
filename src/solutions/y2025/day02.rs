use crate::common::io::read_file;

pub fn run(file_path: &str) -> (u64, u64) {
    let line = read_file(file_path);

    let numbers_stream = line.split(',')
        .map(|id_range| (
            match id_range.split_once('-') {
                None => unreachable!("We fucked up chicken"),
                Some((fi, se)) => (
                    fi.parse::<u64>().unwrap(),
                    se.parse::<u64>().unwrap()
                )
            }))
        .flat_map(|(fi, last)| (fi..last+1));

    let fi = numbers_stream.clone()
            .filter(|&num| {
                let r = radix(num);
                if r % 2 == 1 {
                    return false;
                }
                return checknum(num, radix(num)/2);
            } )
            .sum();
    
    let se = numbers_stream.clone()
            .filter(|&num| (1..radix(num)/2+1).any(|digits| checknum(num, digits)))
            .sum();
    (fi, se)
}

fn radix(mut num: u64) -> u32 {
    let mut res = 0;
    while num > 0 {
        res += 1;
        num /= 10;
    }
    res
}

fn checknum(mut num: u64, check_digits: u32) -> bool {
    if check_digits == 0 {
        return false;
    }

    let pow_ten =  10u64.pow(check_digits);
    let remainder = num%pow_ten;
    let remainder_radix = radix(pow_ten - 1);
    while num > 0 {
        if num%pow_ten != remainder || radix(remainder) != remainder_radix {
            return false;
        }
        num /= pow_ten;
    }
    return true;
}
