// doo will be a super fast command line todo app to pop open when ever your in the command line
// basic functionalities
// list object
//  -- add to a list
//  -- take away from list
//  -- mark as completed
//  -- save a list
//  -- load a list
//  -- open to a specific list via path
//  -- open most recent list
//  -- add subtasks to a task (when we complete all subtasks then we finish the task)
//  -- write an nvim plugin that lets me send todo's to doo
//
//  let's use
//  -- anyhow for errors
//  -- serde for loading and saving to a json file
//  -- tui-rs (crossterm) for display
//  -- have a command line

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = args.get(1);

    doo::run(match filepath {
        Some(path) => {
            eprintln!("found some filepath to load from");
            Some(path.to_string())
        }
        None => None,
    });

    println!("Hello, world!");
}
