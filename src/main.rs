use rand::Rng;

fn initialize_cards() -> Vec<u8> {
    let mut cards: Vec<u8> = Vec::new();
    for i in 1..14 {
        cards.extend(vec![i; 4]);
    }
    cards
}

fn initialize_player<R: Rng>(cards: &mut Vec<u8>, rng: &mut R) -> Option<Vec<u8>> {
    let mut player: Vec<u8> =  Vec::new();
    for _i in 0..2 {
        match hit(cards, rng) {
            Some(card) => {
                player.push(card);
            },
            None => {}
        }
    }
    if player.len()==2 {
        Some(player)
    } else {
        None
    }
}

#[test]
fn initialize_player_test() {
    let mut rng   = rand::thread_rng();
    assert_eq!(initialize_player(&mut vec![], &mut rng), None);
    assert_eq!(initialize_player(&mut vec![1], &mut rng), None);
    assert_eq!(initialize_player(&mut vec![1, 1], &mut rng), Some(vec![1, 1]));
}

fn hit<R: Rng>(cards: &mut Vec<u8>, rng: &mut R) -> Option<u8> {
    if cards.is_empty() {
        None
    } else {
        let index = rng.gen_range(0, cards.len());
        Some(cards.swap_remove(index))
    }
}

#[test]
fn hit_test() {
    let mut rng = rand::thread_rng();
    assert_eq!(hit(&mut Vec::new(), &mut rng), None);
    assert_eq!(hit(&mut vec![], &mut rng),     None);
    assert_eq!(hit(&mut vec![0], &mut rng),    Some(0));
}

fn calculate(mut cards: Vec<u8>) -> u8 {
    let mut result = 0;
    cards.sort_unstable_by(|a, b| b.cmp(a));
    for card in cards {
        if card>=10 {
            result+=10;
        } else if card==1 {
            if result+11>21 {
                result+=1;
            } else {
                result+=11;
            }
        } else {
            result+=card;
        }
    }
    result
}

#[test]
fn calculate_test() {
    assert_eq!(calculate(vec![]),             0);
    assert_eq!(calculate(vec![2]),            2);
    assert_eq!(calculate(vec![1]),            11);
    assert_eq!(calculate(vec![2, 3]),         5);
    assert_eq!(calculate(vec![1, 2]),         13);
    assert_eq!(calculate(vec![13, 13]),       20);
    assert_eq!(calculate(vec![13, 1]),        21);
    assert_eq!(calculate(vec![13, 13, 1]),    21);
    assert_eq!(calculate(vec![13, 13, 13]),   30);
    assert_eq!(calculate(vec![13, 13, 1, 1]), 22);
}

use std::io;
use std::io::Write;
use std::str::FromStr;

fn input<T: FromStr>(s: &str) -> T {
    loop {
        print!("{}", s);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Reading error!");
        match line.trim().parse::<T>() {
            Ok(n) => {
                return n;
            },
            Err(_) => {}
        }
    };
}

fn process_player<R: Rng>(cards: &mut Vec<u8>, player: &mut Vec<u8>, cpu: Vec<u8>, rng: &mut R) {
    println!("[Player turn]");
    println!("Your cards: {:?}({})", player, calculate(player.clone()));
    println!("CPU card:   [{}, ?]", cpu[0]);
    loop {
        let n = input::<u8>("\n[1]: Hit or [0]: Stand: ");
        if n==1 {
            println!("Hit!");
            match hit(cards, rng) {
                Some(card) => {
                    player.push(card);
                },
                _ => {
                    println!("Cards is empty!");
                    std::process::exit(1);
                }
            }
            let result = calculate(player.clone());
            if result>21 {
                println!("You are busted!: {:?}({})", player, result);
                break;
            }
            println!("Your cards: {:?}({})", player, result);
            println!("CPU card:   [{}, ?]", cpu[0]);
        } else if n>1 {
            continue;
        } else {
            println!("Stand!");
            break;
        }
    }
}

fn process_cpu<R: Rng>(cards: &mut Vec<u8>, cpu: &mut Vec<u8>, rng: &mut R) {
    let mut result = calculate(cpu.clone());
    println!("[CPU turn]");
    println!("CPU cards: {:?}({})", cpu, result);
    while result<16 {
        match hit(cards, rng) {
            Some(card) => {
                println!("Hit!");
                cpu.push(card);
                result = calculate(cpu.clone());
            },
            _ => {
                println!("Cards is empty!");
                std::process::exit(1);
            }
        }
    }
    if result>21 {
        println!("CPU are busted!");
    } else {
        println!("Stand!");
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut cards: Vec<u8> = initialize_cards();
    let mut player = initialize_player(&mut cards, &mut rng).expect("Hitting error!");
    let mut cpu    = initialize_player(&mut cards, &mut rng).expect("Hitting error!");
    process_player(&mut cards, &mut player, cpu.clone(), &mut rng);
    if calculate(player.clone())>21 {
        println!("You lose...");
    } else {
        process_cpu(&mut cards, &mut cpu, &mut rng);
        let player_result = calculate(player.clone());
        let cpu_result    = calculate(cpu.clone());
        println!("Your cards: {:?}({})", player, player_result);
        println!("CPU card:   {:?}({})", cpu, cpu_result);
        if cpu_result>21 {
            println!("Player win!!!");
        } else if player_result>cpu_result {
            println!("Player win!!!");
        } else if player_result==cpu_result {
            println!("Draw!");
        } else {
            println!("You lose...");
        }
    }
}
