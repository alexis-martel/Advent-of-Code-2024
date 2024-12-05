fn main() {
    const INPUT: &str = "[INPUT]";
    println!("{}", dec_5_2(INPUT));
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

fn dec_4_1(input: &str) -> i32 {
    let mut xmas_count = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.split_whitespace() {
        let mut new_line: Vec<char> = vec![];
        for character in line.chars() {
            new_line.push(character);
        }
        grid.push(new_line);
    }
    // Find the coordinates of every `X`
    let mut x_coords: Vec<(i32, i32)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                x_coords.push((x as i32, y as i32));
            }
        }
    }
    // Check for XMAS in 8 directions
    for coord in x_coords {
        let x = coord.0 as usize;
        let y = coord.1 as usize;
        // Straight
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid.get(y.wrapping_sub(1)).and_then(|row| row.get(x)) == Some(&'M')
            && grid.get(y.wrapping_sub(2)).and_then(|row| row.get(x)) == Some(&'A')
            && grid.get(y.wrapping_sub(3)).and_then(|row| row.get(x)) == Some(&'S')
        {
            xmas_count += 1;
        }
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid.get(y.wrapping_add(1)).and_then(|row| row.get(x)) == Some(&'M')
            && grid.get(y.wrapping_add(2)).and_then(|row| row.get(x)) == Some(&'A')
            && grid.get(y.wrapping_add(3)).and_then(|row| row.get(x)) == Some(&'S')
        {
            xmas_count += 1;
        }
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid.get(y).and_then(|row| row.get(x.wrapping_sub(1))) == Some(&'M')
            && grid.get(y).and_then(|row| row.get(x.wrapping_sub(2))) == Some(&'A')
            && grid.get(y).and_then(|row| row.get(x.wrapping_sub(3))) == Some(&'S')
        {
            xmas_count += 1;
        }
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid.get(y).and_then(|row| row.get(x.wrapping_add(1))) == Some(&'M')
            && grid.get(y).and_then(|row| row.get(x.wrapping_add(2))) == Some(&'A')
            && grid.get(y).and_then(|row| row.get(x.wrapping_add(3))) == Some(&'S')
        {
            xmas_count += 1;
        }
        // Diagonal
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                == Some(&'M')
            && grid
                .get(y.wrapping_sub(2))
                .and_then(|row| row.get(x.wrapping_add(2)))
                == Some(&'A')
            && grid
                .get(y.wrapping_sub(3))
                .and_then(|row| row.get(x.wrapping_add(3)))
                == Some(&'S')
        {
            xmas_count += 1;
        }
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid
                .get(y.wrapping_sub(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                == Some(&'M')
            && grid
                .get(y.wrapping_sub(2))
                .and_then(|row| row.get(x.wrapping_sub(2)))
                == Some(&'A')
            && grid
                .get(y.wrapping_sub(3))
                .and_then(|row| row.get(x.wrapping_sub(3)))
                == Some(&'S')
        {
            xmas_count += 1;
        }
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_add(1)))
                == Some(&'M')
            && grid
                .get(y.wrapping_add(2))
                .and_then(|row| row.get(x.wrapping_add(2)))
                == Some(&'A')
            && grid
                .get(y.wrapping_add(3))
                .and_then(|row| row.get(x.wrapping_add(3)))
                == Some(&'S')
        {
            xmas_count += 1;
        }
        if grid.get(y).and_then(|row| row.get(x)) == Some(&'X')
            && grid
                .get(y.wrapping_add(1))
                .and_then(|row| row.get(x.wrapping_sub(1)))
                == Some(&'M')
            && grid
                .get(y.wrapping_add(2))
                .and_then(|row| row.get(x.wrapping_sub(2)))
                == Some(&'A')
            && grid
                .get(y.wrapping_add(3))
                .and_then(|row| row.get(x.wrapping_sub(3)))
                == Some(&'S')
        {
            xmas_count += 1;
        }
    }
    xmas_count
}

fn dec_4_2(input: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.split_whitespace() {
        let mut new_line: Vec<char> = vec![];
        for character in line.chars() {
            new_line.push(character);
        }
        grid.push(new_line);
    }
    let mut x_mas_count = 0;
    // Generate every 3*3 sub-matrices possible
    for y in 0..grid.len() - 2 {
        for x in 0..grid.len() - 2 {
            let sub_matrix = [
                [grid[y][x], grid[y][x + 1], grid[y][x + 2]],
                [grid[y + 1][x], grid[y + 1][x + 1], grid[y + 1][x + 2]],
                [grid[y + 2][x], grid[y + 2][x + 1], grid[y + 2][x + 2]],
            ];
            let c1 = (sub_matrix[0][0] == 'M'
                && sub_matrix[1][1] == 'A'
                && sub_matrix[2][2] == 'S')
                || (sub_matrix[0][0] == 'S' && sub_matrix[1][1] == 'A' && sub_matrix[2][2] == 'M');
            let c2 = (sub_matrix[0][2] == 'M'
                && sub_matrix[1][1] == 'A'
                && sub_matrix[2][0] == 'S')
                || (sub_matrix[0][2] == 'S' && sub_matrix[1][1] == 'A' && sub_matrix[2][0] == 'M');
            if c1 && c2 {
                x_mas_count += 1;
            }
        }
    }
    x_mas_count
}

fn dec_5_1(input: &str) -> i32 {
    let mut sum = 0;
    // Process input into vectors
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let mut rules: Vec<(i32, i32)> = vec![];
    for line in split_input[0].split_whitespace() {
        let split_line: Vec<&str> = line.split("|").collect();
        rules.push((
            split_line[0].parse().unwrap(),
            split_line[1].parse().unwrap(),
        ));
    }
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in split_input[1].split_whitespace() {
        let split_line: Vec<&str> = line.split(",").collect();
        let mut update: Vec<i32> = vec![];
        for page_str in split_line {
            update.push(page_str.parse().unwrap());
        }
        updates.push(update);
    }
    // Check rule adhesion
    for update in updates {
        let mut correct = true;
        let mut applicable_rules: Vec<(i32, i32)> = vec![];
        for rule in &rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                applicable_rules.push(rule.to_owned());
            }
        }
        for rule in applicable_rules {
            let first_page_pos = update.iter().position(|&r| r == rule.0).unwrap();
            let second_page_pos = update.iter().position(|&r| r == rule.1).unwrap();
            if first_page_pos > second_page_pos {
                correct = false;
            }
        }
        if correct {
            sum += update[(update.len() - 1) / 2];
        }
    }
    sum
}

fn dec_5_2(input: &str) -> i32 {
    let mut sum = 0;
    // Process input into vectors
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let mut rules: Vec<(i32, i32)> = vec![];
    for line in split_input[0].split_whitespace() {
        let split_line: Vec<&str> = line.split("|").collect();
        rules.push((
            split_line[0].parse().unwrap(),
            split_line[1].parse().unwrap(),
        ));
    }
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in split_input[1].split_whitespace() {
        let split_line: Vec<&str> = line.split(",").collect();
        let mut update: Vec<i32> = vec![];
        for page_str in split_line {
            update.push(page_str.parse().unwrap());
        }
        updates.push(update);
    }
    // Check rule adhesion
    fn is_correct(update: &[i32], rules: &Vec<(i32, i32)>) -> (bool, usize, usize) {
        let mut correct = true;
        let mut applicable_rules: Vec<(i32, i32)> = vec![];
        for rule in rules {
            if update.contains(&rule.0) && update.contains(&rule.1) {
                applicable_rules.push(rule.to_owned());
            }
        }
        for rule in applicable_rules {
            let first_page_pos = update.iter().position(|&r| r == rule.0).unwrap();
            let second_page_pos = update.iter().position(|&r| r == rule.1).unwrap();
            if first_page_pos > second_page_pos {
                correct = false;
                return (correct, first_page_pos, second_page_pos);
            }
        }
        (correct, 0, 0)
    }
    for update in updates {
        if !is_correct(&update, &rules).0 {
            let mut copy = update.clone();
            // Part 2 code here
            let mut is_correct_res = is_correct(&copy, &rules);
            while !is_correct_res.0 {
                // Shuffle
                copy.swap(is_correct_res.1, is_correct_res.2);
                is_correct_res = is_correct(&copy, &rules);
            }
            sum += copy[(update.len() - 1) / 2];
        }
    }
    sum
}
