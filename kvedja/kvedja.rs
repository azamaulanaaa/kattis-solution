fn main() {
    let mut lines = std::io::stdin().lines().map(|x| x.unwrap());
    let value = lines.next().unwrap();
    println!("Kvedja,\n{}", value);
}
