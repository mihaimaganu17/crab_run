fn main() {
    std::thread::spawn(|| println!("Greeetings, people"))
        .join()
        .unwrap()
}
