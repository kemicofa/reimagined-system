use std::process;
use colored::*;
use git::Git;

const MAIN: &str = "main";
const MASTER: &str = "master";

pub fn run() {
    let current_branch = Git::get_current_branch_name();

    if current_branch == MAIN || current_branch == MASTER {
        println!("{}", "Should never attempt to push to the main/master branch".red());
        process::exit(1);
    }

    Git::push(&current_branch);
}