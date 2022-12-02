pub fn part_one(input: &str) -> Option<u32> {
    let input_split_nl: Vec<&str> = input.rsplit('\n').collect();
    let mut calories_single_elf: u32 = 0;
    let mut highest_calories: u32 = 0;
    let mut i = 1;
    let len = input_split_nl.len();
    for line in input_split_nl {
        if !line.is_empty() {
            let food_item = line.parse::<u32>().unwrap();
            calories_single_elf += food_item
        }
        if line.is_empty() || i == len {
            if calories_single_elf > highest_calories {
                highest_calories = calories_single_elf;
            }
            calories_single_elf = 0
        }
        i += 1
    }
    Some(highest_calories)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_split_nl: Vec<&str> = input.rsplit('\n').collect();
    let mut calories_single_elf: u32 = 0;
    let mut first_highest_calories: u32 = 0;
    let mut second_highest_calories: u32 = 0;
    let mut third_highest_calories: u32 = 0;
    let mut i = 1;
    let len = input_split_nl.len();
    for line in input_split_nl {
        if !line.is_empty() {
            let food_item = line.parse::<u32>().unwrap();
            calories_single_elf += food_item
        }
        if line.is_empty() || i == len {
            if calories_single_elf >= first_highest_calories {
                third_highest_calories = second_highest_calories;
                second_highest_calories = first_highest_calories;
                first_highest_calories = calories_single_elf;
            } else if calories_single_elf >= second_highest_calories {
                third_highest_calories = second_highest_calories;
                second_highest_calories = calories_single_elf;
            } else if calories_single_elf > third_highest_calories {
                third_highest_calories = calories_single_elf
            }
            calories_single_elf = 0
        }
        i += 1
    }
    let total_calories = first_highest_calories + second_highest_calories + third_highest_calories;
    Some(total_calories)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(2000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(8000));
    }
}
