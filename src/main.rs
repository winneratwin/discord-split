use std::io;
use std::fs::File;
use std::io::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
   /// Number of characters to have in each segment.
   #[clap(long, value_parser, default_value_t = 2000)]
   split_at: u32,

   /// Default file prefix.
   #[clap(long, value_parser, default_value = "discord-split")]
   file_prefix: String,

   /// write to file
   #[clap(long, value_parser, default_value_t = false)]
   write_to_file: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut results: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let lines = io::stdin().lines();
    let mut first_word = false;
    for line in lines {
        // read line to variable
        let inline = line.unwrap().trim().to_string();
        // check if line is bigger then 2000 characters and if so split it
        if (inline.len()+1) as u32 > args.split_at
        {
            let res:Vec<&str> = inline.split(' ').collect();
            for x in res {
                // check if the "word" is longer then the length it is going to be split into
                if x.len() as u32 > args.split_at
                {
                    // seperate into length chunks
                    let result = sub_strings(x, args.split_at as usize);
                    for x in result {
                        //if the buffer + word is longer then the split number 
                        if (buffer.len() + x.len()) as u32 > args.split_at
                        {
                            buffer = buffer.trim().to_string();
                            buffer.push('\n');
                            results.push(buffer);
                            buffer = x.to_owned();
                            println!("{:?}",buffer);
                        } else { // if buffer+word is less then split number just add it
                            if !first_word {
                                buffer.push(' ');
                            } else {
                                first_word=false;
                            }
                            buffer.push_str(x);
                            buffer.push(' ');
                        }
                    }
                }
                //if the buffer + word is longer then the split number 
                if (buffer.len() + x.len()) as u32 > args.split_at
                {
                    buffer=buffer.trim().to_string();
                    buffer.push('\n');
                    results.push(buffer);
                    first_word=false;
                    buffer = x.to_owned();
                } else { // if buffer+word is less then split number just add it
                    println!("{:?}",buffer);
                    if !first_word {
                        buffer.push(' ');
                    } else {
                        first_word=false;
                    }
                    buffer.push_str(x);
                    println!("{:?}",buffer);
                }
            }
            buffer.push('\n');
            first_word=true;
            continue;
        }
        // check if buffer + current line would be bigger then 2000 characters (discord without
        // nitros limit
        if (buffer.len() + inline.len()) as u32 > args.split_at
        {
            buffer=buffer.trim().to_string();
            buffer.push('\n');
            results.push(buffer);
            buffer = inline;
            buffer.push('\n');
            first_word=true;
        }
        // else append current line to buffer
        else
        {
            buffer.push_str(inline.as_str());
            buffer.push('\n');
            first_word=true;
        }
    }
    buffer = buffer.trim().to_owned();
    buffer.push('\n');
    results.push(buffer);
    let zeropad_length = format!("{}",results.len()).len();
    let mut counter = 0;
    for contents in results
    {
        counter += 1;
        if args.write_to_file
        {
            let mut f = File::create(format!("{}-{:0width$}.txt",args.file_prefix, counter,width = zeropad_length))?;
            f.write_all(contents.as_bytes())?;
        } else {
            println!("-------------------------------");
            println!("{}",contents);
            println!("-------------------------------");
        }
    }
    Ok(())
}

// by juggle-tux at https://users.rust-lang.org/t/solved-how-to-split-string-into-multiple-sub-strings-with-given-length/10542/8
fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}
