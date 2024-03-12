fn main() {
    let cmd = std::env::args().nth(1).expect("no command given");

    println!("cmd: {:?}", cmd);
}
