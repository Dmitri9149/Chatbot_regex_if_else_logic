use std::io;
use rand::Rng;


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
            and will reply to you soon.\n");

    let random_number = rand::thread_rng().gen_range(1,6);

    if random_number == 1 {
        println!("By the way, what is your favourite greeting ? Hey ? Hello ?");
    } else if random_number == 2 {
        println!("....hmmm.... to think about it takes a longer time than I expected...");
    } else if random_number == 3 {
        println!(".....just a moment....please...");
    } else if random_number == 4 {
        println!("By the way, do you think the 'Rosa' is a good name for a bot ?")     
    } else {
        println!("OK ! I almoust ready. But if you have something to finilise...like \
                your university course in Quantum Mechanics, may be it is a good idea \
                for you to do it now, you still have some time.\n");
    }      



}
