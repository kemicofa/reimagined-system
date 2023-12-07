use std::process::Command;
use colored::Colorize;
use consts::{GIT_CURRENT_BRANCH, GIT_PUSH_REMOTE};
use regex::Regex;
use utils::get_stdout_or_terminate;

use crate::consts::{GIT_VERSION, GIT_PUSH_REMOTE_REGEX};

mod consts;
mod utils;

pub struct Git;

impl Git {
    pub fn get_current_branch_name() -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(GIT_CURRENT_BRANCH)
            .output()
            .expect("Unable to retrieve current branch");

        get_stdout_or_terminate(&output)
    }

    pub fn get_version() -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg(GIT_VERSION)
            .output()
            .expect("Expected to be able to run `git -v` command");

            get_stdout_or_terminate(&output)
    }

    pub fn push(remote_branch_name: &str) {
        let re = Regex::new(GIT_PUSH_REMOTE_REGEX).unwrap();
        let command = re.replace(GIT_PUSH_REMOTE, remote_branch_name).to_string();

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect(format!("Expected to be able to run `{}` command", command).as_str());

        println!("{}", get_stdout_or_terminate(&output).bright_black());
    }
}
