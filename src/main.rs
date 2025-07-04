fn main() {
    const INPUT: &str = "";
    println!("{}", dec_7_1(INPUT));
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
fn dec_6_1(input: &str) -> i32 {
    fn move_guard(x: usize, y: usize, direction: char) -> (usize, usize) {
        let mut new_x = x;
        let mut new_y = y;
        match direction {
            'U' => new_y = new_y.wrapping_sub(1),
            'D' => new_y += 1,
            'L' => new_x = new_x.wrapping_sub(1),
            'R' => new_x += 1,
            _ => return (new_x, new_y),
        };
        (new_x, new_y)
    }
    fn change_direction(direction: char) -> char {
        match direction {
            'U' => 'R',
            'D' => 'L',
            'L' => 'U',
            'R' => 'D',
            _ => panic!(),
        }
    }
    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.split_whitespace() {
        let mut new_line: Vec<char> = vec![];
        for char in line.chars() {
            new_line.push(char);
        }
        grid.push(new_line);
    }
    let mut guard_x: usize = 0;
    let mut guard_y: usize = 0;
    let mut guard_direction = 'U';
    // Find guard coords
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                guard_x = x;
                guard_y = y;
            }
        }
    }
    // Move guard
    loop {
        let old_coords = (guard_x, guard_y);
        let new_coords = move_guard(guard_x, guard_y, guard_direction);
        if new_coords.0 < grid[0].len() && new_coords.1 < grid.len() {
            if grid[new_coords.1][new_coords.0] == '#' {
                guard_direction = change_direction(guard_direction);
            } else {
                grid[old_coords.1][old_coords.0] = 'X';
                guard_x = new_coords.0;
                guard_y = new_coords.1;
            }
        } else {
            grid[old_coords.1][old_coords.0] = 'X';
            break;
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                sum += 1;
            }
        }
    }
    sum
}

fn dec_6_2(input: &str) -> i32 {
    fn move_guard(x: usize, y: usize, direction: char) -> (usize, usize) {
        let mut new_x = x;
        let mut new_y = y;
        match direction {
            'U' => new_y = new_y.wrapping_sub(1),
            'D' => new_y += 1,
            'L' => new_x = new_x.wrapping_sub(1),
            'R' => new_x += 1,
            _ => return (new_x, new_y),
        };
        (new_x, new_y)
    }
    fn change_direction(direction: char) -> char {
        match direction {
            'U' => 'R',
            'D' => 'L',
            'L' => 'U',
            'R' => 'D',
            _ => panic!(),
        }
    }
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.split_whitespace() {
        let mut new_line: Vec<char> = vec![];
        for char in line.chars() {
            new_line.push(char);
        }
        grid.push(new_line);
    }
    let mut guard_x: usize;
    let mut guard_y: usize;
    let mut guard_start_x: usize = 0;
    let mut guard_start_y: usize = 0;
    let mut guard_direction;
    // Find guard coords
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                guard_start_x = x;
                guard_start_y = y;
            }
        }
    }
    // Generate every possible grid
    let mut modded_grids: Vec<Vec<Vec<char>>> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '^' {
                let mut mod_grid = grid.clone();
                mod_grid[y][x] = '#';
                modded_grids.push(mod_grid);
            }
        }
    }
    const HUGE_NUMBER: i128 = 1e6 as i128;
    let mut looping_obstacle_locations = 0;
    let mut i = 1;
    let i_max = grid.len() * grid[0].len();
    #[allow(clippy::explicit_counter_loop)]
    for mut modded_grid in modded_grids {
        // Move guard
        guard_x = guard_start_x;
        guard_y = guard_start_y;
        guard_direction = 'U';
        let mut loop_counter: i128 = 0;
        loop {
            let old_coords = (guard_x, guard_y);
            let new_coords = move_guard(guard_x, guard_y, guard_direction);
            if new_coords.0 < modded_grid[0].len() && new_coords.1 < modded_grid.len() {
                if modded_grid[new_coords.1][new_coords.0] == '#' {
                    guard_direction = change_direction(guard_direction);
                } else {
                    modded_grid[old_coords.1][old_coords.0] = 'X';
                    guard_x = new_coords.0;
                    guard_y = new_coords.1;
                }
            } else {
                modded_grid[old_coords.1][old_coords.0] = 'X';
                break;
            }
            loop_counter += 1;
            if loop_counter >= HUGE_NUMBER {
                looping_obstacle_locations += 1;
                break;
            }
        }
        println!("{}/{}", i, i_max);
        i += 1;
    }
    looping_obstacle_locations
}

fn dec_7_1(input: &str) -> i64 {
    fn evaluate_expression(expr: &str) -> i64 {
        let mut result = 0;
        let mut current_number = 0;
        let mut current_operator = '+';

        for c in expr.chars() {
            if c.is_digit(10) {
                current_number = current_number * 10 + c.to_digit(10).unwrap() as i64;
            } else if "+-*/".contains(c) {
                match current_operator {
                    '+' => result += current_number,
                    '-' => result -= current_number,
                    '*' => result *= current_number, // If you want to include multiplication
                    '/' => result /= current_number, // If you want to include division
                    _ => {}
                }
                current_operator = c;
                current_number = 0;
            }
        }
        // Apply the last operation
        match current_operator {
            '+' => result += current_number,
            '-' => result -= current_number,
            '*' => result *= current_number, // If you want to include multiplication
            '/' => result /= current_number, // If you want to include division
            _ => {}
        }
        result
    }
    struct Equation<'a> {
        result: i64,
        numbers: Vec<&'a str>,
    }
    let mut calibration_result = 0;
    // Parse input
    let mut equations: Vec<Equation> = vec![];
    for line in input.split("\n") {
        let split_line: Vec<&str> = line.split(": ").collect();
        let res = split_line[0].parse::<i64>().unwrap();
        let nums: Vec<&str> = split_line[1].split_whitespace().collect::<Vec<&str>>();
        equations.push(Equation {
            result: res,
            numbers: nums,
        });
    }
    let eq_amount = equations.len();
    for (count, eq) in equations.into_iter().enumerate() {
        let mut max_combination_binary_num_str: String = "".to_owned();
        let mut combinations_bin_num_str: Vec<String> = vec![];
        for _i in 0..eq.numbers.len() - 1 {
            max_combination_binary_num_str.push('1');
        }
        let max_combination_num =
            i64::from_str_radix(&max_combination_binary_num_str, 2).unwrap() + 1;
        for i in 0..max_combination_num {
            let formatted_binary = format!("{:b}", i);
            let mut leading_zeros = "".to_owned();
            for _i in 0..max_combination_binary_num_str.len() - formatted_binary.len() {
                leading_zeros.push('0');
            }
            combinations_bin_num_str.push(leading_zeros + &formatted_binary as &str + "e");
        }
        let mut to_evaluate: Vec<String> = vec![];
        for bin_num_str in combinations_bin_num_str {
            // Generate combination in `to_evaluate`
            let mut combination: String = "".to_owned();
            for i in 0..eq.numbers.len() {
                combination.push_str(eq.numbers[i]);
                if bin_num_str.chars().collect::<Vec<char>>()[i] == '0' {
                    combination.push('*');
                } else if bin_num_str.chars().collect::<Vec<char>>()[i] == '1' {
                    combination.push('+');
                }
            }
            to_evaluate.push(combination);
        }
        for evaluate_me in &to_evaluate {
            let res = evaluate_expression(evaluate_me);
            if res == eq.result {
                calibration_result += res;
                break;
            }
        }
        println!("{}/{}", count + 1, eq_amount);
    }
    calibration_result
}
