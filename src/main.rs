#![allow(unused)]

use std::io;
use std::fs::File;
use std::io::prelude::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
   /// Number of characters to have in each segment.
   #[clap(short, value_parser, default_value_t = 2000)]
   count: u32,

   /// Default file prefix.
   #[clap(short, value_parser, default_value = "discord-split")]
   prefix: String,

   /// Print to console.
   #[clap(long, value_parser, default_value_t = false)]
   console: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let mut results: Vec<String> = Vec::new();
    let mut buffer = String::new();
    let lines = io::stdin().lines();
    for line in lines {
        // read line to variable
        let inline = line.unwrap();
        // check if line is bigger then 2000 characters and drop it
        if inline.len() as u32 > args.count
        {
            let res:Vec<&str> = inline.split(' ').collect();
            for x in res
            {
                if x.len() as u32 > args.count
                {
                    println!("{} was dropped because it was longer then {}",x,args.count);
                    continue;
                }
                if (buffer.len() + x.len()) as u32 > args.count
                {
                    buffer.push('\n');
                    results.push(buffer);
                    buffer = x.to_owned();
                } else {
                    buffer.push_str(x);
                    buffer.push(' ');
                }
            }

        }
        // check if buffer + current line would be bigger then 2000 characters (discord without
        // nitros limit
        if (buffer.len() + inline.len()) as u32 >= args.count
        {
            results.push(buffer);
            buffer = inline;
            buffer.push('\n');
        }
        // else append current line to buffer
        else
        {
            buffer.push_str(inline.as_str());
            buffer.push('\n');
        }
    }
    results.push(buffer);
    let zeropad_length = format!("{}",results.len()).len();
    let mut counter = 0;
    for contents in results
    {
        counter += 1;
        if !args.console
        {
            let mut f = File::create(format!("{}-{:0width$}.txt",args.prefix, counter,width = zeropad_length))?;
            f.write_all(contents.as_bytes())?;
        } else {
            println!("{}",contents);
            println!("-------------------------------");
        }
    }
    Ok(())
}
