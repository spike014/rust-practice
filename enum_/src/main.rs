#[derive(Debug)]
enum PokerCard {
    _Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    _Hearts(u8),
}

fn main() {
    let c1 = PokerCard::Spades(5);
    let c2 = PokerCard::Diamonds(13);
    println!("{:?}", c1);
    println!("{:?}", c2);

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("none.");
            None
        }
        Some(i) => {
            println!("{}", i);
            Some(i + 1)
        }
    }
}

// Option<T>
// zh-CN: https://rustwiki.org/zh-CN/std/option/enum.Option.html
// en: https://doc.rust-lang.org/std/option/enum.Option.html
