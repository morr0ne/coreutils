use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rustix::process::{Gid, Uid};

use crate::Result;

#[derive(Debug)]
pub struct Passwd<'a> {
    pub name: &'a str,
    pub passwd: &'a str,
    pub uid: Uid,
    pub gid: Gid,
    pub gecos: &'a str,
    pub dir: &'a str,
    pub shell: &'a str,
}

pub struct Parser<R> {
    reader: R,
    buf: Vec<u8>,
}

impl Parser<BufReader<File>> {
    pub fn new() -> Result<Self> {
        let file = File::open("/etc/passwd")?;
        let reader = BufReader::new(file);

        Ok(Self {
            reader,
            buf: Vec::new(),
        })
    }

    pub fn next_entry(&mut self) -> Result<Option<Passwd>> {
        self.buf.clear();
        let n = self.reader.read_until(b'\n', &mut self.buf)?;

        if n != 0 {
            if self.buf.last() == Some(&b'\n') {
                self.buf.pop();
            }

            Ok(Passwd::from_buf(&self.buf))
        } else {
            Ok(None)
        }
    }
}

impl<'a> Passwd<'a> {
    fn from_buf(buf: &'a [u8]) -> Option<Self> {
        /*
        TODO
        Possibily rethink the whole implementation. The current one is *techincally* correct but I am not sure it's the most efficient.
        It is possible the compiler is optimizing it but that is questionable especially in the uid and gid case.
        Also its not very readable and there's is a lot of repeatition.
        I feel like this could be refactored somehow but I'm gonna leave it as is for now since it works.
         */

        let mut entries = buf.splitn(7, |s| s == &b':');

        let name = entries
            .next()
            .map(|bytes| std::str::from_utf8(bytes))?
            .ok()?;

        let passwd = entries
            .next()
            .map(|bytes| std::str::from_utf8(bytes))?
            .ok()?;

        let uid = entries
            .next()
            .map(|bytes| std::str::from_utf8(bytes))?
            .ok()?
            .parse()
            .map(|n| unsafe { Uid::from_raw(n) })
            .ok()?;

        let gid = entries
            .next()
            .map(|bytes| std::str::from_utf8(bytes))?
            .ok()?
            .parse()
            .map(|n| unsafe { Gid::from_raw(n) })
            .ok()?;

        let gecos = entries
            .next()
            .map(|bytes| std::str::from_utf8(bytes))?
            .ok()?;

        let dir = entries
            .next()
            .map(|bytes| std::str::from_utf8(bytes))?
            .ok()?;

        let shell = entries
            .next()
            .map(|bytes| std::str::from_utf8(bytes))?
            .ok()?;

        Some(Passwd {
            name,
            passwd,
            uid,
            gid,
            gecos,
            dir,
            shell,
        })
    }
}
