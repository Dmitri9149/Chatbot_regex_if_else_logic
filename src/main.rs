use std::io;


fn main() {
    println!("I am a bot! Let us discuss!\n");
    println!("My name is Rosa.\n");
    println!("I am using a simple regular expressions\nand all power of my mind\nto analise \
            what you will say to me.\n");

    let mut request = String :: new();
    println!("You may say something like 'Hello' to me: {}.\n", request);

    io::stdin()
        .read_line(&mut request)
        .expect("Something is wrong with reading the line, failed to read it");


    println!("I see you say {} to me!", request);
    println!("It is very vise from your side,\nI will think about it a little bit \
            and will reply to you soon.") 


}
