/*
TODO
Calling println in a loop like this is inefficient to say the least. It is a very *very* bad way to implement yes.
While it does techically work it's not how it should be done and is in fact not how gnu coreutils does it.
The gnu implementation is actually much smarter than this and does some fancy stuff which buffer and direct usage of libc.
Eventually this will implemented more optimaly but it is essentially the correct behaviour beside the speed.
*/

use std::{borrow::Cow, env::Args};

use clap::{Arg, ArgAction};

use crate::{util::new_command, Result};

pub fn yes(args: Args) -> Result {
    let matches = new_command(
        "yes",
        "Repeatedly output a line with all specified STRING(s), or 'y'.",
        
    )
    .arg(Arg::new("STRING").action(ArgAction::Append))
    .get_matches_from(args);

    let value = if let Some(values) = matches.get_many::<String>("STRING") {
        let value: String = values.flat_map(|s| s.chars()).collect();
        Cow::Owned(value)
    } else {
        Cow::Borrowed("y")
    };

    loop {
        println!("{value}")
    }
}
