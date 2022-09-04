use proconio::input;

fn main() {
    input! {
        s: String
    };

    match s.as_str() {
        "Monday" => {
            println!("5");
        },
        "Tuesday" => {
            println!("4");
        },
        "Wednesday" => {
            println!("3");
        },
        "Thursday" => {
            println!("2");
        },
        "Friday" => {
            println!("1");
        },
        _ => {
            panic!("unexpected value")
        },

    }
}
