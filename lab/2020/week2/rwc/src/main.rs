use std::env;
use std::io;
use std::io::BufRead;
use std::process;
use std::fs::File;

fn main()-> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)

    let file = File::open(filename).expect("Open file failed,please check the path!\n");

    let mut line_num = 0;
    let mut word_num = 0;
    let mut char_num = 0;

    for line in io::BufReader::new(file).lines(){
        let line_str = line?;
        line_num += 1;
        char_num += line_str.len();
        word_num += line_str.split_whitespace().count();
    }

    println!("Word count: {}", word_num);
    println!("Line count: {}", line_num);
    println!("Character count: {}", char_num);

    Ok(())

}