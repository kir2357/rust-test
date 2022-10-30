#![warn(dead_code)]

fn test1() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);

    // 以下と等しい
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    println!("{:?}", logicians);
}

fn test2() {
    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }

    println!("{:?}", logicians);
}

#[test]
fn main() {
    println!("test1");
    test1();
    println!("test2");
    test2();
}
