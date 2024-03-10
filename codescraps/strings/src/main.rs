fn first_word(text: &str) -> &str {
    let mut index = 0;

    for(i, item) in text.chars().enumerate() {
        if item == ' ' {
            return &text[0..i]; // Wenn gefunden, gib den Text vom ersten bis zum Leerzeichen
            // zurück
        }
        index = i;
    }
    // Wenn nicht gefunden, gib den ganzen Text zurück;
    &text[0..index]
}

fn main() {
    let real_string: String = String::from("Hello World!");
    let x = first_word(&real_string);
    println!("{}", x);

    let literal: &str = "Hello World!";
    let y = first_word(&literal);
    let z = first_word(literal);
    println!("{}", y);
    println!("{}", z);

}
