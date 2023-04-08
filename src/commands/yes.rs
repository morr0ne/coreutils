/*
TODO
Calling println in a loop like this is inefficient to say the least. It is a very *very* bad way to implement yes.
While it does techically work it's not how it should be done and is in fact not how gnu coreutils does it.
The gnu implementation is actually much smarter than this and does some fancy stuff which buffer and direct usage of libc.
Eventually this will implemented more optimaly but it is essentially the correct behaviour beside the speed.
*/

use std::{
    borrow::Cow,
    env::Args,
    io::{stdout, Write},
    process::ExitCode,
};

use lexopt::prelude::*;

use crate::Result;

pub fn yes(args: Args) -> Result {
    let string = {
        let mut string: Option<String> = None;
        let mut parser = lexopt::Parser::from_args(args);

        while let Some(arg) = parser.next()? {
            match arg {
                Value(value) => {
                    if let Some(string) = &mut string {
                        string.push_str(" ");
                        string.push_str(&value.string()?)
                    } else {
                        string = Some(value.string()?);
                    }
                }
                Long("help") => {
                    println!("Usage: yes");
                    return Ok(ExitCode::SUCCESS);
                }
                _ => return Err(arg.unexpected().into()),
            }
        }

        string.map(|s| Cow::Owned(s)).unwrap_or(Cow::Borrowed("y"))
    };

    let mut stdout = stdout().lock();

    loop {
        writeln!(&mut stdout, "{string}")?
    }
}
