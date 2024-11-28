// #![allow(unused)]
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, BufRead};
// use std::time::Instant;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Player {
    id: usize,
    games_amount: usize,
}

fn _test() {
    // // TEST1
    // let p1 = Player {
    //     id: 1,
    //     games_amount: 1,
    // };
    // let p2 = Player {
    //     id: 2,
    //     games_amount: 2,
    // };
    // let p3 = Player {
    //     id: 3,
    //     games_amount: 3,
    // };
    // let p4 = Player {
    //     id: 4,
    //     games_amount: 1,
    // };
    // let p5 = Player {
    //     id: 5,
    //     games_amount: 2,
    // };
    // let p6 = Player {
    //     id: 6,
    //     games_amount: 3,
    // };
    // let p7 = Player {
    //     id: 7,
    //     games_amount: 1,
    // };
    // let p8 = Player {
    //     id: 8,
    //     games_amount: 2,
    // };
    // let p9 = Player {
    //     id: 9,
    //     games_amount: 3,
    // };
    // let player_list = Vec::from([p1, p2, p3, p4, p5, p6, p7, p8, p9]);
    // _pretty_print_test(&player_list);
    // println!("Now we battle!\n");
    // let battle_list = battle_algorithm(player_list);
    // println!("{:?}", battle_list);

    // pretty_print_test(battle_list);
}

fn main() {
    // _test();
    // let now = Instant::now();
    let (player_amount, player_list) = read_input();
    let (message, game_list) = battle_algorithm(player_list);
    // let elapsed_time = now.elapsed();
    // println!("Spend {} seconds on processing", elapsed_time.as_secs_f64());

    println!("{}", message);
    if let Some(list) = game_list {
        print_battle_list(player_amount, list);
    }
}

fn print_battle_list(list_length: usize, indexed_list: HashMap<usize, BinaryHeap<usize>>) {
    for id in 1..=list_length {
        let current_string: &String = &indexed_list[&id]
            .clone()
            .into_sorted_vec()
            .iter()
            .map(|&x| x.to_string() + " ")
            .collect();
        println!("{}", current_string);
    }
}

fn _print_binary_heap(list: &BinaryHeap<usize>) {
    let number_string: String = list
        .clone()
        .into_sorted_vec()
        .iter()
        .map(|&x| x.to_string() + " ")
        .collect();
    println!("{}", number_string);
}

fn read_input() -> (usize, Vec<Player>) {
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();
    let players_amount: usize = input_lines
        .next()
        .expect("stdin not available")
        .expect("couldn't read input")
        .trim()
        .parse()
        .expect("not a positive integer");

    let mut player_list = Vec::new();
    for player_id in 1..=players_amount {
        let last_person: usize = input_lines
            .next()
            .expect("stdin not available")
            .expect("couldn't read input")
            .parse()
            .expect("Not an integer");
        let last_person = Player {
            id: player_id,
            games_amount: last_person,
            // opponents: BinaryHeap::new(),
        };
        player_list.push(last_person);
    }

    return (players_amount, player_list);
}

fn process_battle(
    current_player: &Player,
    player_list: &mut Vec<Player>,
    opponent_index: usize,
    battle_list: &mut HashMap<usize, BinaryHeap<usize>>,
) {
    let opponent_id = player_list[opponent_index].id;
    battle_list
        .entry(opponent_id)
        .or_default()
        .push(current_player.id);
    player_list[opponent_index].games_amount -= 1;
    battle_list
        .entry(current_player.id)
        .or_default()
        .push(opponent_id);
}

fn battle_algorithm(
    mut player_list: Vec<Player>,
) -> (String, Option<HashMap<usize, BinaryHeap<usize>>>) {
    player_list.sort_by(|a, b| a.games_amount.cmp(&b.games_amount));
    let mut players_count = player_list.len();
    let mut battle_list: HashMap<usize, BinaryHeap<usize>> = HashMap::new();
    for index in 1..=players_count {
        battle_list.insert(index, BinaryHeap::new());
    }
    loop {
        let current_player = match player_list.pop() {
            None => {
                let message = String::from("SOLUTION");
                return (message, Some(battle_list));
            }
            Some(player) => player,
        };
        players_count -= 1;
        if current_player.games_amount <= 0 {
            let message = String::from("SOLUTION");
            return (message, Some(battle_list));
        }
        // target = first we don't subtract from, before reordering
        // Check if enough players left to play agains
        if current_player.games_amount > players_count {
            let message = String::from("NO SOLUTION");
            return (message, None);
        }

        let target = players_count + 1 - current_player.games_amount;
        let target_value = player_list[target - 1].games_amount;
        if target_value <= 0 {
            let message = String::from("NO SOLUTION");
            return (message, None);
        }
        let target_low;
        let mut opponent_index = players_count - 1;
        let mut games_processed = 0;
        loop {
            if player_list[opponent_index].games_amount == target_value {
                break;
            }
            process_battle(
                &current_player,
                &mut player_list,
                opponent_index,
                &mut battle_list,
            );
            games_processed += 1;
            opponent_index -= 1;
        }
        loop {
            if player_list[opponent_index].games_amount != target_value {
                target_low = opponent_index + 1;
                break;
            }
            if opponent_index == 0 {
                target_low = 0;
                break;
            }
            opponent_index -= 1;
        }
        let games_missing = current_player.games_amount - games_processed;
        for opponent_index in target_low..target_low + games_missing {
            process_battle(
                &current_player,
                &mut player_list,
                opponent_index,
                &mut battle_list,
            )
        }
    }
}
