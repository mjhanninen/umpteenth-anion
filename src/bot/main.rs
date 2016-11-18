// Copyright (C) 2016 Matti Hänninen
//
// This file is part of Umpteenth Anion.
//
// Umpteenth Anion is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Umpteenth Anion is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
// for more details.
//
// You should have received a copy of the GNU General Public License along
// with Umpteenth Anion.  If not, see <http://www.gnu.org/licenses/>.

#![allow(non_snake_case)]

extern crate getopts;
extern crate ua;

use std::io::Write;
use std::env;

mod simple;

enum Brain
{
    Simple,
}

struct Config
{
    brain: Brain,
    log_path: Option<String>,
}

enum OptionParsing
{
    ShowUsage(String),
    Failed,
    Config(Config),
}

fn parse_options() -> OptionParsing
{
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "Display this usage information")
        .optopt("b", "brain", "Select the bot brain to use", "NAME")
        .optopt("l", "log", "Produce log of internal events to file", "FILE");
    let args = env::args().collect::<Vec<String>>();
    if let Ok(matches) = opts.parse(&args[1..]) {
        if matches.opt_present("h") {
            let brief = format!("usage: {} [ options ]", args[0]);
            OptionParsing::ShowUsage(opts.usage(&brief))
        } else if !matches.free.is_empty() {
            OptionParsing::Failed
        } else {
            let mut config = Config {
                brain: Brain::Simple,
                log_path: None,
            };
            if let Some(log_path) = matches.opt_str("l") {
                config.log_path = Some(log_path);
            }
            OptionParsing::Config(config)
        }
    } else {
        OptionParsing::Failed
    }
}

fn main()
{
    match parse_options() {
        OptionParsing::Config(config) => {
            match config.brain {
                Brain::Simple => simple::run()
            }
        }
        OptionParsing::ShowUsage(usage) => {
            println!("{}", usage);
            std::process::exit(0);
        }
        OptionParsing::Failed => {
            writeln!(std::io::stderr(),
                     "Error: Bad command line (try -h for help)")
                .unwrap();
            std::process::exit(1);
        }
    }
}
