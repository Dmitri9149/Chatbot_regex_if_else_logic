#![warn(missing_docs, unused_variables)]

extern crate regex;

use std::io;
use rand::Rng;
use regex::RegexSetBuilder;
use regex::RegexSet;
use regex::Regex;
use std::collections::HashSet;
#[macro_use] extern crate lazy_static;

fn main() {
    println!("I am a bot! Let us discuss!\n");
    println!("My name is Rosa.\n");
    println!("I am using a simple regular expressions and all power of my mind\nto analise \
            what you will say to me.\n");
   
// use white space insignificant mode (?x) that is why use '\x20' asw hite space
//

    lazy_static! {
        static ref RE:Regex = Regex::new(
            r"(?x)(?i)
                [^a-z]*
                ([y]o|[h']?ello|ok|hey|(good[\x20])?(morn[gin']{0,3}
                |afternoon|day|even[gin']{0,3}))
                [\s,;:]{1,3}
                (?P<rosa_name>[a-z]{1,20})"
        ).unwrap();
    }

    let mut rosa_good_names = HashSet::new();
        rosa_good_names.insert("rose".to_string());
        rosa_good_names.insert("rosa".to_string());
        rosa_good_names.insert("chatty".to_string());
        rosa_good_names.insert("chatbot".to_string());
        rosa_good_names.insert("bot".to_string());
        rosa_good_names.insert("chatterbot".to_string());
        rosa_good_names.insert("rosi".to_string());

    let mut rosa_curt_names = HashSet::new();
        rosa_curt_names.insert("hal".to_string());
        rosa_curt_names.insert("you".to_string());
        rosa_curt_names.insert("u".to_string());
        rosa_curt_names.insert("yu".to_string());

    let mut request = String :: new();
    println!("You may say something like 'Hello' to me: {}.\n", &mut request);

    io::stdin()
        .read_line(&mut request)
        .expect("Something is wrong with reading the line, failed to read it");


// the 'greeter_name' is not used in the version
    let greeter_name = &"";

    let capts = RE.captures(&mut request).unwrap();
    let rosa_name = capts.name("rosa_name").unwrap().as_str();

    if rosa_curt_names.contains(rosa_name) {
        println!("Good name\n")
    } else if rosa_good_names.contains(rosa_name) {
        println!("Hi {}, How are you ?\n", greeter_name)
    } else {
        println!("Please, name me as 'Rosa', I like it\n")
    }        

    println!("I see you say {} to me!\n", &mut request);
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
