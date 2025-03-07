// #![allow(unused)]
// use std::collections::HashMap;
// use std::hash::Hash
use std::io::{self, BufRead /*, Write*/};

fn main() {
    let (size, players) = read_input();
    // println!("A list of games played by players, in order: {:?}", players);

    let indexed_players = index_players(players);
    // println!(
    //     "Tuples indicating number of games each player \
    //     played: {:?}",
    //     indexed_players
    // );
    let (message, battlelist) = battles(size, indexed_players);
    // save result in "result"
    // writeln!(io::stdout(), "{}", result).expect("oh no");
    // vec_to_string(
    // println!("------------------------------");
    let output = battles_to_string(&message, &battlelist);
    println!("{}", output)
}

fn read_input() -> (usize, Vec<isize>) {
    let mut output_strings: Vec<isize> = Vec::new();
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let size = lines.next().expect("error").expect("errrroor");
    let size = size.parse::<usize>().expect("not a number");
    for _index in 0..size {
        let last_person = lines.next().expect("arg").expect("argx2");
        let last_person = last_person.parse::<isize>().expect("not a number");
        output_strings.push(last_person)
    }

    return (size, output_strings);
}

fn index_players(players: Vec<isize>) -> Vec<(usize, isize)> {
    let mut players_indexed: Vec<(usize, isize)> = Vec::new();

    let mut index = 0;
    for player in players {
        players_indexed.push((index, player));
        index += 1
    }
    players_indexed.sort_by(|a, b| a.1.cmp(&b.1));

    return players_indexed;
}

fn battles(size: usize, mut indexed_players: Vec<(usize, isize)>) -> (String, Vec<Vec<usize>>) {
    // entry: (player number, list of players they play against)
    // let mut battle_list: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut message = String::from("SOLUTION");
    let mut battle_list: Vec<Vec<usize>> = Vec::new();
    // So player id's goes from 0 to N-1!!
    for _player_id in 0..size {
        battle_list.push(Vec::new());
    }
    loop {
        let (current_player_id, games) = indexed_players.pop().unwrap();
        if indexed_players.is_empty() {
            if games > 0 {
                // println!("Last player has noone to play agains ;(");
                message = String::from("NO SOLUTION");
            }
            break;
        }
        // println!("Now the players left are {:?}", indexed_players);
        let len = indexed_players.len();
        //
        // This is where to check if no solution! If this fails, no solution
        if len as isize - games < 0 {
            // println!("This player has not enough players to play against ;(");
            message = String::from("NO SOLUTION");
            break;
        }
        let lower_index = len - games as usize;
        //
        // let len = len as usize;
        // let lower_bound_games = indexed_players[games];
        // let lower_bound_index = indexed_players.partition_point(|(x,y)| y > lower_bound_games);
        for index in lower_index..len {
            let opponent_id = indexed_players[index].0;
            // println!(
            //     "Current player {current_player_id} plays \
            //     agains opponent {opponent_id}"
            // );
            if indexed_players[index].1 == 0 {
                // println!("Player {opponent_id} has to play too many games ;(");
                message = String::from("NO SOLUTION");
                break;
            }
            indexed_players[index].1 -= 1;
            // Add one so player 0 becomes player 1!
            battle_list[current_player_id].push(opponent_id + 1);
            battle_list[opponent_id].push(current_player_id + 1);
            // THIS IS THE BOTTLENECK!!!!!
            // INSTEAD LOOK AHEAD, AND JUST REMOVE IN A WAY SO LIST REMAINS
            // SORTED!
            indexed_players.sort_by(|a, b| a.1.cmp(&b.1));
        }
    }
    // dbg!(&battle_list);
    // println!("{message}");
    return (message, battle_list);
}

fn vec_to_string(number_list: &Vec<usize>) -> String {
    let mut output_string = String::new();
    let mut number_list = number_list.clone();
    number_list.sort();
    for number in number_list.iter() {
        let number = number.to_string();
        output_string.push(' ');
        output_string.push_str(&number);
    }
    // output_string = String::from(output_string.trim());
    return output_string;
}

fn battles_to_string(message: &String, battle_list: &Vec<Vec<usize>>) -> String {
    if message == &String::from("NO SOLUTION") {
        return String::from("NO SOLUTION");
    }
    let mut output_string = String::new();
    output_string.push_str(message);
    for list in battle_list {
        output_string.push('\n');
        output_string.push_str(&vec_to_string(list));
    }

    return output_string;
}

fn distribute_games(player_id: usize, mut indexed_players: Vec<usize, isize>) {
    for index in lower_index..len {
        let opponent_id = indexed_players[index].0;
        // println!(
        //     "Current player {current_player_id} plays \
        //     agains opponent {opponent_id}"
        // );
        if indexed_players[index].1 == 0 {
            // println!("Player {opponent_id} has to play too many games ;(");
            message = String::from("NO SOLUTION");
            break;
        }
        indexed_players[index].1 -= 1;
        // Add one so player 0 becomes player 1!
        battle_list[current_player_id].push(opponent_id + 1);
        battle_list[opponent_id].push(current_player_id + 1);
        // THIS IS THE BOTTLENECK!!!!!
        // INSTEAD LOOK AHEAD, AND JUST REMOVE IN A WAY SO LIST REMAINS
        // SORTED!
        indexed_players.sort_by(|a, b| a.1.cmp(&b.1));
    }
}
