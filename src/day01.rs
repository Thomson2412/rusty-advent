pub fn execute(input: &str) -> i32 {
    let lines = read_input(input);
    let mut current_position = 50;
    let mut zero_position_amount = 0;

    for line in lines {
        current_position = rotate(line, current_position);
        if current_position == 0 {
            zero_position_amount += 1;
        }
    }

    return zero_position_amount;
}

fn rotate(command: &str, position: i32) -> i32 {
    let mut new_position = position;
    let (direction, amount_str) = command.split_at(1);
    let amount: i32 = amount_str.parse().unwrap();
    match direction {
        "R" => {
            new_position += amount;
        }
        "L" => {
            new_position -= amount;
        }
        _ => {}
    }
    return new_position % 100;
}

fn read_input(input: &str) -> std::str::Lines<'_> {
    let lines = input.lines();
    return lines;
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROTATION_LIST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_right_rotation_50_gives_1() {
        assert_eq!(execute("R50"), 1);
    }

    #[test]
    fn test_right_rotation_250_gives_1() {
        assert_eq!(execute("R250"), 1);
    }

    #[test]
    fn test_right_rotation_25_25_gives_1() {
        assert_eq!(execute("R25\nR25"), 1);
    }

    #[test]
    fn test_left_rotation_25_25_gives_1() {
        assert_eq!(execute("L25\nL25"), 1);
    }

    #[test]
    fn test_left_rotation_50_gives_1() {
        assert_eq!(execute("L50"), 1);
    }
    #[test]
    fn test_left_rotation_250_gives_1() {
        assert_eq!(execute("R250"), 1);
    }

    #[test]
    fn test_l50_l25_r25_gives_2() {
        assert_eq!(execute("L50\nL25\nR25"), 2);
    }

    #[test]
    fn test_execute_returns_3_for_rotation_list() {
        assert_eq!(execute(ROTATION_LIST), 3);
    }
}
