fn main() {
    for i in 1..101 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("CracklePop");
        }
        else if i % 3 == 0 {
            println!("Crackle");
        }
        else if i % 5 == 0 {
            println!("Pop");
        }
        else {
            println!("{i}");
        }
    }
}
