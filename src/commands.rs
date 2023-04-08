#[cfg(feature = "arch")]
mod arch;
#[cfg(feature = "b2sum")]
mod b2sum;
#[cfg(feature = "base32")]
mod base32;
#[cfg(feature = "base64")]
mod base64;
#[cfg(feature = "basename")]
mod basename;
#[cfg(feature = "basenc")]
mod basenc;
#[cfg(feature = "cat")]
mod cat;
#[cfg(feature = "chcon")]
mod chcon;
#[cfg(feature = "chgrp")]
mod chgrp;
#[cfg(feature = "chmod")]
mod chmod;
#[cfg(feature = "chown")]
mod chown;
#[cfg(feature = "chroot")]
mod chroot;
#[cfg(feature = "cksum")]
mod cksum;
#[cfg(feature = "comm")]
mod comm;
#[cfg(feature = "cp")]
mod cp;
#[cfg(feature = "csplit")]
mod csplit;
#[cfg(feature = "cut")]
mod cut;
#[cfg(feature = "date")]
mod date;
#[cfg(feature = "dd")]
mod dd;
#[cfg(feature = "df")]
mod df;
#[cfg(feature = "dir")]
mod dir;
#[cfg(feature = "dircolors")]
mod dircolors;
#[cfg(feature = "dirname")]
mod dirname;
#[cfg(feature = "du")]
mod du;
#[cfg(feature = "echo")]
mod echo;
#[cfg(feature = "env")]
mod env;
#[cfg(feature = "expand")]
mod expand;
#[cfg(feature = "expr")]
mod expr;
#[cfg(feature = "factor")]
mod factor;
#[cfg(feature = "false")]
mod r#false;
#[cfg(feature = "fmt")]
mod fmt;
#[cfg(feature = "fold")]
mod fold;
#[cfg(feature = "groups")]
mod groups;
#[cfg(feature = "head")]
mod head;
#[cfg(feature = "hostid")]
mod hostid;
#[cfg(feature = "hostname")]
mod hostname;
#[cfg(feature = "id")]
mod id;
#[cfg(feature = "install")]
mod install;
#[cfg(feature = "join")]
mod join;
#[cfg(feature = "kill")]
mod kill;
#[cfg(feature = "link")]
mod link;
#[cfg(feature = "ln")]
mod ln;
#[cfg(feature = "logname")]
mod logname;
#[cfg(feature = "ls")]
mod ls;
#[cfg(feature = "md5sum")]
mod md5sum;
#[cfg(feature = "mkdir")]
mod mkdir;
#[cfg(feature = "mkfifo")]
mod mkfifo;
#[cfg(feature = "mknod")]
mod mknod;
#[cfg(feature = "mktemp")]
mod mktemp;
#[cfg(feature = "mv")]
mod mv;
#[cfg(feature = "nice")]
mod nice;
#[cfg(feature = "nl")]
mod nl;
#[cfg(feature = "nohup")]
mod nohup;
#[cfg(feature = "nproc")]
mod nproc;
#[cfg(feature = "numfmt")]
mod numfmt;
#[cfg(feature = "od")]
mod od;
#[cfg(feature = "paste")]
mod paste;
#[cfg(feature = "pathchk")]
mod pathchk;
#[cfg(feature = "pinky")]
mod pinky;
#[cfg(feature = "pr")]
mod pr;
#[cfg(feature = "printenv")]
mod printenv;
#[cfg(feature = "printf")]
mod printf;
#[cfg(feature = "ptx")]
mod ptx;
#[cfg(feature = "pwd")]
mod pwd;
#[cfg(feature = "readlink")]
mod readlink;
#[cfg(feature = "realpath")]
mod realpath;
#[cfg(feature = "rm")]
mod rm;
#[cfg(feature = "rmdir")]
mod rmdir;
#[cfg(feature = "runcon")]
mod runcon;
#[cfg(feature = "seq")]
mod seq;
#[cfg(feature = "sha1sum")]
mod sha1sum;
#[cfg(feature = "sha224sum")]
mod sha224sum;
#[cfg(feature = "sha256sum")]
mod sha256sum;
#[cfg(feature = "sha384sum")]
mod sha384sum;
#[cfg(feature = "sha512sum")]
mod sha512sum;
#[cfg(feature = "shred")]
mod shred;
#[cfg(feature = "shuf")]
mod shuf;
#[cfg(feature = "sleep")]
mod sleep;
#[cfg(feature = "sort")]
mod sort;
#[cfg(feature = "split")]
mod split;
#[cfg(feature = "stat")]
mod stat;
#[cfg(feature = "stdbuf")]
mod stdbuf;
#[cfg(feature = "stty")]
mod stty;
#[cfg(feature = "sum")]
mod sum;
#[cfg(feature = "sync")]
mod sync;
#[cfg(feature = "tac")]
mod tac;
#[cfg(feature = "tail")]
mod tail;
#[cfg(feature = "tee")]
mod tee;
#[cfg(feature = "test")]
mod test;
#[cfg(feature = "timeout")]
mod timeout;
#[cfg(feature = "touch")]
mod touch;
#[cfg(feature = "tr")]
mod tr;
#[cfg(feature = "true")]
mod r#true;
#[cfg(feature = "truncate")]
mod truncate;
#[cfg(feature = "tsort")]
mod tsort;
#[cfg(feature = "tty")]
mod tty;
#[cfg(feature = "uname")]
mod uname;
#[cfg(feature = "unexpand")]
mod unexpand;
#[cfg(feature = "uniq")]
mod uniq;
#[cfg(feature = "unlink")]
mod unlink;
#[cfg(feature = "uptime")]
mod uptime;
#[cfg(feature = "users")]
mod users;
#[cfg(feature = "vdir")]
mod vdir;
#[cfg(feature = "wc")]
mod wc;
#[cfg(feature = "who")]
mod who;
#[cfg(feature = "whoami")]
mod whoami;
#[cfg(feature = "yes")]
mod yes;

#[cfg(feature = "arch")]
pub use arch::arch;
#[cfg(feature = "b2sum")]
pub use b2sum::b2sum;
#[cfg(feature = "base32")]
pub use base32::base32;
#[cfg(feature = "base64")]
pub use base64::base64;
#[cfg(feature = "basename")]
pub use basename::basename;
#[cfg(feature = "basenc")]
pub use basenc::basenc;
#[cfg(feature = "cat")]
pub use cat::cat;
#[cfg(feature = "chcon")]
pub use chcon::chcon;
#[cfg(feature = "chgrp")]
pub use chgrp::chgrp;
#[cfg(feature = "chmod")]
pub use chmod::chmod;
#[cfg(feature = "chown")]
pub use chown::chown;
#[cfg(feature = "chroot")]
pub use chroot::chroot;
#[cfg(feature = "cksum")]
pub use cksum::cksum;
#[cfg(feature = "comm")]
pub use comm::comm;
#[cfg(feature = "cp")]
pub use cp::cp;
#[cfg(feature = "csplit")]
pub use csplit::csplit;
#[cfg(feature = "cut")]
pub use cut::cut;
#[cfg(feature = "date")]
pub use date::date;
#[cfg(feature = "dd")]
pub use dd::dd;
#[cfg(feature = "df")]
pub use df::df;
#[cfg(feature = "dir")]
pub use dir::dir;
#[cfg(feature = "dircolors")]
pub use dircolors::dircolors;
#[cfg(feature = "dirname")]
pub use dirname::dirname;
#[cfg(feature = "du")]
pub use du::du;
#[cfg(feature = "echo")]
pub use echo::echo;
#[cfg(feature = "env")]
pub use env::env;
#[cfg(feature = "expand")]
pub use expand::expand;
#[cfg(feature = "expr")]
pub use expr::expr;
#[cfg(feature = "factor")]
pub use factor::factor;
#[cfg(feature = "fmt")]
pub use fmt::fmt;
#[cfg(feature = "fold")]
pub use fold::fold;
#[cfg(feature = "groups")]
pub use groups::groups;
#[cfg(feature = "head")]
pub use head::head;
#[cfg(feature = "hostid")]
pub use hostid::hostid;
#[cfg(feature = "hostname")]
pub use hostname::hostname;
#[cfg(feature = "id")]
pub use id::id;
#[cfg(feature = "install")]
pub use install::install;
#[cfg(feature = "join")]
pub use join::join;
#[cfg(feature = "kill")]
pub use kill::kill;
#[cfg(feature = "link")]
pub use link::link;
#[cfg(feature = "ln")]
pub use ln::ln;
#[cfg(feature = "logname")]
pub use logname::logname;
#[cfg(feature = "ls")]
pub use ls::ls;
#[cfg(feature = "md5sum")]
pub use md5sum::md5sum;
#[cfg(feature = "mkdir")]
pub use mkdir::mkdir;
#[cfg(feature = "mkfifo")]
pub use mkfifo::mkfifo;
#[cfg(feature = "mknod")]
pub use mknod::mknod;
#[cfg(feature = "mktemp")]
pub use mktemp::mktemp;
#[cfg(feature = "mv")]
pub use mv::mv;
#[cfg(feature = "nice")]
pub use nice::nice;
#[cfg(feature = "nl")]
pub use nl::nl;
#[cfg(feature = "nohup")]
pub use nohup::nohup;
#[cfg(feature = "nproc")]
pub use nproc::nproc;
#[cfg(feature = "numfmt")]
pub use numfmt::numfmt;
#[cfg(feature = "od")]
pub use od::od;
#[cfg(feature = "paste")]
pub use paste::paste;
#[cfg(feature = "pathchk")]
pub use pathchk::pathchk;
#[cfg(feature = "pinky")]
pub use pinky::pinky;
#[cfg(feature = "pr")]
pub use pr::pr;
#[cfg(feature = "printenv")]
pub use printenv::printenv;
#[cfg(feature = "printf")]
pub use printf::printf;
#[cfg(feature = "ptx")]
pub use ptx::ptx;
#[cfg(feature = "pwd")]
pub use pwd::pwd;
#[cfg(feature = "false")]
pub use r#false::r#false;
#[cfg(feature = "true")]
pub use r#true::r#true;
#[cfg(feature = "readlink")]
pub use readlink::readlink;
#[cfg(feature = "realpath")]
pub use realpath::realpath;
#[cfg(feature = "rm")]
pub use rm::rm;
#[cfg(feature = "rmdir")]
pub use rmdir::rmdir;
#[cfg(feature = "runcon")]
pub use runcon::runcon;
#[cfg(feature = "seq")]
pub use seq::seq;
#[cfg(feature = "sha1sum")]
pub use sha1sum::sha1sum;
#[cfg(feature = "sha224sum")]
pub use sha224sum::sha224sum;
#[cfg(feature = "sha256sum")]
pub use sha256sum::sha256sum;
#[cfg(feature = "sha384sum")]
pub use sha384sum::sha384sum;
#[cfg(feature = "sha512sum")]
pub use sha512sum::sha512sum;
#[cfg(feature = "shred")]
pub use shred::shred;
#[cfg(feature = "shuf")]
pub use shuf::shuf;
#[cfg(feature = "sleep")]
pub use sleep::sleep;
#[cfg(feature = "sort")]
pub use sort::sort;
#[cfg(feature = "split")]
pub use split::split;
#[cfg(feature = "stat")]
pub use stat::stat;
#[cfg(feature = "stdbuf")]
pub use stdbuf::stdbuf;
#[cfg(feature = "stty")]
pub use stty::stty;
#[cfg(feature = "sum")]
pub use sum::sum;
#[cfg(feature = "sync")]
pub use sync::sync;
#[cfg(feature = "tac")]
pub use tac::tac;
#[cfg(feature = "tail")]
pub use tail::tail;
#[cfg(feature = "tee")]
pub use tee::tee;
#[cfg(feature = "test")]
pub use test::test;
#[cfg(feature = "timeout")]
pub use timeout::timeout;
#[cfg(feature = "touch")]
pub use touch::touch;
#[cfg(feature = "tr")]
pub use tr::tr;
#[cfg(feature = "truncate")]
pub use truncate::truncate;
#[cfg(feature = "tsort")]
pub use tsort::tsort;
#[cfg(feature = "tty")]
pub use tty::tty;
#[cfg(feature = "uname")]
pub use uname::uname;
#[cfg(feature = "unexpand")]
pub use unexpand::unexpand;
#[cfg(feature = "uniq")]
pub use uniq::uniq;
#[cfg(feature = "unlink")]
pub use unlink::unlink;
#[cfg(feature = "uptime")]
pub use uptime::uptime;
#[cfg(feature = "users")]
pub use users::users;
#[cfg(feature = "vdir")]
pub use vdir::vdir;
#[cfg(feature = "wc")]
pub use wc::wc;
#[cfg(feature = "who")]
pub use who::who;
#[cfg(feature = "whoami")]
pub use whoami::whoami;
#[cfg(feature = "yes")]
pub use yes::yes;
