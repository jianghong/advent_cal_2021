fn main() {
    let mut player_1_score = 0;
    let mut player_1_position = 2;
    
    let mut player_2_score = 0;
    let mut player_2_position = 7;
    const MAX_SCORE: u64 = 21;
    let mut roll_count = 0;
    while player_1_score < MAX_SCORE && player_2_score < MAX_SCORE {
        roll_count += 1;
        let roll = calculate_roll(roll_count);
        player_1_position = calculate_new_position(player_1_position, roll);
        player_1_score += player_1_position;
        if player_1_score >= MAX_SCORE {
            break;
        }
        roll_count += 1;
        let roll = calculate_roll(roll_count);
        player_2_position = calculate_new_position(player_2_position, roll);
        player_2_score += player_2_position;
        println!("Player 1 score: {}", player_1_score);
        println!("Player 1 position: {}", player_1_position);
        println!("Player 2 score: {}", player_2_score);
        println!("Player 2 position: {}", player_2_position);
    }
    roll_count *= 3;
    println!("Player 1 score: {}", player_1_score);
    println!("Player 2 score: {}", player_2_score);
    println!("roll count: {}", roll_count);
    println!("part 1 {}", roll_count * std::cmp::min(player_1_score, player_2_score));
}

fn part1() {
    let mut player_1_score = 0;
    let mut player_1_position = 2;
    
    let mut player_2_score = 0;
    let mut player_2_position = 7;
    const MAX_SCORE: u64 = 1000;
    let mut roll_count = 0;
    while player_1_score < MAX_SCORE && player_2_score < MAX_SCORE {
        roll_count += 1;
        let roll = calculate_roll(roll_count);
        player_1_position = calculate_new_position(player_1_position, roll);
        player_1_score += player_1_position;
        if player_1_score >= MAX_SCORE {
            break;
        }
        roll_count += 1;
        let roll = calculate_roll(roll_count);
        player_2_position = calculate_new_position(player_2_position, roll);
        player_2_score += player_2_position;
        println!("Player 1 score: {}", player_1_score);
        println!("Player 1 position: {}", player_1_position);
        println!("Player 2 score: {}", player_2_score);
        println!("Player 2 position: {}", player_2_position);
    }
    roll_count *= 3;
    println!("Player 1 score: {}", player_1_score);
    println!("Player 2 score: {}", player_2_score);
    println!("roll count: {}", roll_count);
    println!("part 1 {}", roll_count * std::cmp::min(player_1_score, player_2_score));
}

fn calculate_roll(roll_count: u64) -> u64 {
    let mod_val = 100;
    let mut sum = 0;
    let mut max = (roll_count * 3) % mod_val;
    for _ in 0..3 {
        if max == 0 {
            max = mod_val;
        }
        sum += max;
        max -= 1;
    }
    return sum
}

fn calculate_new_position(current_position: u64, roll: u64) -> u64 {
    let mod_val = 10;
    let new_position = (current_position + roll) % mod_val;
    if new_position == 0 {
        return mod_val;
    } else {
        return new_position;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_roll() {
        assert_eq!(calculate_roll(1), 6);
        assert_eq!(calculate_roll(2), 15);
        assert_eq!(calculate_roll(33), 294);
        assert_eq!(calculate_roll(34), 103);
    }

    #[test]
    fn test_calculate_new_position() {
        assert_eq!(calculate_new_position(4, calculate_roll(1)), 10);
    }
}