fn main() {
    const INPUT: &str = "[INPUT]";
    println!("{}", dec_3_2(INPUT));
}

fn dec_1_1(input: &str) -> i32 {
    let mut list_1: Vec<i32> = vec![];
    let mut list_2: Vec<i32> = vec![];
    for line in input.split("\n") {
        let cols = line.split_whitespace().collect::<Vec<&str>>();
        list_1.push(cols[0].parse::<i32>().unwrap());
        list_2.push(cols[1].parse::<i32>().unwrap());
    }
    list_1.sort();
    list_2.sort();
    assert_eq!(list_1.len(), list_2.len());
    let mut total_distance = 0;
    for i in 0..list_1.len() {
        total_distance += (list_1[i] - list_2[i]).abs();
    }
    total_distance
}

fn dec_1_2(input: &str) -> i32 {
    let mut list_1: Vec<i32> = vec![];
    let mut list_2: Vec<i32> = vec![];
    for line in input.split("\n") {
        let cols = line.split_whitespace().collect::<Vec<&str>>();
        list_1.push(cols[0].parse::<i32>().unwrap());
        list_2.push(cols[1].parse::<i32>().unwrap());
    }
    let mut list_1_uniques: Vec<i32> = vec![];
    for item in list_1 {
        if !list_1_uniques.contains(&item) {
            list_1_uniques.push(item);
        }
    }
    let mut similarity_score = 0;
    for unique_item in list_1_uniques {
        let mut occurences = 0;
        for item in &list_2 {
            if *item == unique_item {
                occurences += 1;
            }
        }
        similarity_score += unique_item * occurences;
    }
    similarity_score
}

fn dec_2_1(input: &str) -> i32 {
    let mut safe_count = 0;
    for line in input.split("\n") {
        let report_str_nums = line.split_whitespace().collect::<Vec<&str>>();
        let mut report: Vec<i32> = vec![];
        for i in report_str_nums {
            report.push(i.parse().unwrap());
        }
        let mut safe = true;
        for i in 1..report.len() {
            if !(1..=3).contains(&(report[i] - report[i - 1]).abs()) {
                safe = false;
            }
        }
        if !(report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b)) {
            safe = false;
        }
        if safe {
            safe_count += 1;
        }
    }
    safe_count
}

fn dec_2_2(input: &str) -> i32 {
    let mut safe_count = 0;
    for line in input.split("\n") {
        let report_str_nums = line.split_whitespace().collect::<Vec<&str>>();
        let mut report: Vec<i32> = vec![];
        for i in report_str_nums {
            report.push(i.parse().unwrap());
        }
        // Check if already safe
        let mut safe = true;
        for i in 1..report.len() {
            if !(1..=3).contains(&(report[i] - report[i - 1]).abs()) {
                safe = false;
            }
        }
        if !(report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b)) {
            safe = false;
        }
        if !safe {
            let mut damps: Vec<Vec<i32>> = vec![];
            for i in 0..report.len() {
                let mut rep = report.clone();
                rep.remove(i);
                damps.push(rep);
            }
            let mut at_least_one_safe_damp = false;
            for dampened_report in damps {
                let mut damp_is_safe = true;
                for i in 1..dampened_report.len() {
                    if !(1..=3).contains(&(dampened_report[i] - dampened_report[i - 1]).abs()) {
                        damp_is_safe = false;
                    }
                }
                if !(dampened_report.is_sorted_by(|a, b| a < b)
                    || dampened_report.is_sorted_by(|a, b| a > b))
                {
                    damp_is_safe = false;
                }
                at_least_one_safe_damp = at_least_one_safe_damp || damp_is_safe;
            }
            if at_least_one_safe_damp {
                safe_count += 1;
            }
        } else {
            safe_count += 1;
        }
    }
    safe_count
}
fn dec_3_1(input: &str) -> i32 {
    use regex::Regex;
    let re = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut sum = 0;
    for mul_expression in matches {
        println!("{}", mul_expression);
        let nums = mul_expression.replace("mul(", "").replace(")", "");
        let split_nums: Vec<&str> = nums.split(",").collect();
        sum += split_nums[0].parse::<i32>().unwrap() * split_nums[1].parse::<i32>().unwrap();
    }
    sum
}

fn dec_3_2(input: &str) -> i32 {
    use regex::Regex;
    let re = Regex::new(r"mul\([0-9]+\,[0-9]+\)").unwrap();
    let matches: Vec<_> = re.find_iter(input).map(|m| m.as_str()).collect();
    let mut mul_indices = vec![];
    for mat in re.find_iter(input) {
        mul_indices.push((mat.start(), mat.end()));
    }
    let mut sum = 0;
    for i in 0..matches.len() {
        println!(
            "{} from {} to {}",
            matches[i], mul_indices[i].0, mul_indices[i].1
        );
        let search_slice = &input[0..mul_indices[i].0];
        let last_do_index = search_slice.match_indices("do()").last();
        let last_dont_index = search_slice.match_indices("don't()").last();
        if last_dont_index > last_do_index {
            // Don't do mul
        } else {
            // Do mul
            let nums = matches[i].replace("mul(", "").replace(")", "");
            let split_nums: Vec<&str> = nums.split(",").collect();
            sum += split_nums[0].parse::<i32>().unwrap() * split_nums[1].parse::<i32>().unwrap();
        }
    }
    sum
}
