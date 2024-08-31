fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "こんにちは";
    let china = "你好";
    let emojiland = "Hello 👋🌍";

    let regions = [southern_germany, japan, china, emojiland];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
