fn duplicate_character_count(first: &str, second: &str) -> usize {
    second
        .chars()
        .filter(|c| first.contains(*c))
        .count()
}

fn main() {
    println!("{}", duplicate_character_count("aloha", "hei"));
    println!("{}", duplicate_character_count("jambo", "bonjour"));
    println!("{}", duplicate_character_count("hello", "hola"));
    println!("{}", duplicate_character_count("ola", "hej"));
    println!("{}", duplicate_character_count("ciao", "konnichiwa"));
    println!("{}", duplicate_character_count("merhaba", "xin chao"));
    println!(
        "{}",
        duplicate_character_count(
            "hello world",
            "hello to everyone around the world"
        )
    ); 
}