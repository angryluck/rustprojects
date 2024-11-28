use std::io::{self, BufRead};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Passenger {
    seat: usize,
    time: isize,
}

fn _test() {
    // 6
    // 3 10
    // 1 3
    // 2 8
    // 5 12
    // 4 5
    // 6 2
    let _test_size: usize = 6;
    let _test_passengers = vec![
        Passenger { seat: 3, time: 10 },
        Passenger { seat: 1, time: 3 },
        Passenger { seat: 2, time: 8 },
        Passenger { seat: 5, time: 12 },
        Passenger { seat: 4, time: 5 },
        Passenger { seat: 6, time: 2 },
    ];
    dbg!(
        _test_size,
        &_test_passengers,
        &_test_passengers[1].seat,
        &_test_passengers[2].time
    );
    dbg!(seating_time(_test_passengers));
}

fn main() {
    _test();
    // let (passenger_amount, mut passengers) = read_input();
    // dbg!(seating_time(passengers));
}

fn read_input() -> (usize, Vec<Passenger>) {
    // This line is needed in rust 1.4
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();
    let passenger_amount: usize = input_lines
        .next()
        .expect("stdin not available")
        .expect("couldn't read input")
        .trim()
        .parse()
        .expect("not a positive integer");
    let mut passenger_list = Vec::new();
    for _ in 1..=passenger_amount {
        let current_input: String = input_lines
            .next()
            .expect("stdin not available")
            .expect("couldn't read input");
        let mut current_input = current_input.split(' ');
        let seat: usize = current_input
            .next()
            .expect("No number was input")
            .parse()
            .expect("Not a number");
        let time: isize = current_input
            .next()
            .expect("Only one number was input")
            .parse()
            .expect("Not a number");
        let current_passenger = Passenger { time, seat };

        passenger_list.push(current_passenger);
    }

    return (passenger_amount, passenger_list);
}

fn seating_time(passengers: Vec<Passenger>) -> isize {
    let mut total_time = 0;
    let mut seated_list: Vec<Passenger> = Vec::new();
    for passenger in passengers {
        total_time += passenger.time;
        // let seating = seated_list.binary_search_by(|x| x.seat.cmp(&passenger.seat) );
        let seating = seated_list.binary_search(&passenger);
        match seating {
            Err(seating) => {
                let mut del_indexes = Vec::new();
                for i in 0..seating {
                    seated_list[i].time -= passenger.time;
                    if seated_list[i].time <= 0 { del_indexes.push(i) }
                }
                seated_list.insert(seating, passenger);
            }
            Ok(seating) => seated_list.insert(seating, passenger),
        }
    }
    dbg!(seated_list);

    total_time
}
