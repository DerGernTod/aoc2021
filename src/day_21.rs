pub fn part_1() {
    let (loser_score, die_rolls) = play_game_deterministic(6, 2);
    println!("Final result: {}", loser_score * die_rolls);
}

pub fn part_2() {
    let score = play_game_random(6, 2);
    println!("The winner wins in {} universes!", score);
}

struct Player(u8, u8);

fn play_game_random(p1_start: u8, p2_start: u8) -> u64 {
    // in 1 turn a player can move 3-9 fields. possibilities per move:
    // 3: 1 (1-1-1)
    // 4: 3 (2-1-1, 1-2-1, 1-1-2)
    // 5: 6 (3-1-1, 1-3-1, 1-1-3, 2-2-1, 2-1-2, 1-2-2)
    // 6: 7
    // 7: 6
    // 8: 3
    // 9: 1

    let move_positions: [u8; 7] = [1, 3, 6, 7, 6, 3, 1];
    let (p1_wins, p2_wins) = check_all_winners_after_turn(&Player(p1_start, 0), &Player(p2_start, 0), 0, move_positions);
    u64::max(p1_wins, p2_wins)
}

fn check_all_winners_after_turn(p1: &Player, p2: &Player, turn: usize, move_positions: [u8; 7]) -> (u64, u64) {
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let player = if turn % 2 == 0 { p1 } else { p2 };

    for (move_index, count) in move_positions.iter().enumerate() {
        let move_amount = move_index as u8 + 3;
        let result_position = (player.0 + move_amount) % 10;
        let result_position = if result_position == 0 { 10 } else { result_position };
        let result_score = player.1 + result_position;
        let (p1_add, p2_add) = match (result_score >= 21, turn % 2 == 0) {
            (true, true) => (1, 0),
            (true, false) => (0, 1),
            (false, true) => check_all_winners_after_turn(&Player(result_position, result_score), p2, turn + 1, move_positions),
            (false, false) => check_all_winners_after_turn(p1, &Player(result_position, result_score), turn + 1, move_positions),
        };
        p1_wins += p1_add * *count as u64;
        p2_wins += p2_add * *count as u64;
    }
    (p1_wins, p2_wins)
}

fn play_game_deterministic(p1_start: u32, p2_start: u32) -> (u32, u32) {
    let mut p1_score = 0;
    let mut p1_location = p1_start;
    let mut p2_score = 0;
    let mut p2_location = p2_start;
    let mut die_offset = 0;
    let mut p1s_turn = true;
    while p1_score < 1000 && p2_score < 1000 {
        if p1s_turn {
            let result_position = roll_die_deterministic(p1_location, die_offset);
            p1_score += result_position;
            p1_location = result_position;
        } else {
            let result_position = roll_die_deterministic(p2_location, die_offset);
            p2_score += result_position;
            p2_location = result_position;
        }
        die_offset += 3;
        p1s_turn = !p1s_turn;
    }
    (u32::min(p1_score, p2_score), die_offset)
}

fn roll_die_deterministic(board_offset: u32, die_offset: u32) -> u32 {
    let moves = (die_offset + 1) * 3 + 3;
    let result = (board_offset + moves) % 10;
    if result == 0 {
        10
    } else {
        result
    }
}


#[cfg(test)]
mod tests {
    use crate::day_21::*;
    #[test]
    fn test_part_1() {
        let (loser_score, die_rolls) = play_game_deterministic(4, 8);
        assert_eq!(loser_score, 745);
        assert_eq!(die_rolls, 993);
        assert_eq!(loser_score * die_rolls, 739785);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(444356092776315, play_game_random(4, 8));
    }
    #[test]
    fn test_roll_die() {
        let res = roll_die_deterministic(10, 0);
        assert_eq!(res, 6);
        let res = roll_die_deterministic(2, 0);
        assert_eq!(res, 8);
        let res = roll_die_deterministic(8, 9);
        assert_eq!(res, 1);
        let res = roll_die_deterministic(2, 4);
        assert_eq!(res, 10);
    }
}