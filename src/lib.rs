#![recursion_limit="512"]
extern crate time;
//extern crate nix;
extern crate rand;
extern crate find_folder;
#[macro_use]
extern crate conrod;
extern crate image;
extern crate winapi;
extern crate xml_rpc;

pub mod cenario;
pub mod dados;
pub mod par;
pub mod respostaSN;
pub mod sessao;
pub mod teste;
pub mod variavel;
pub mod gui;
pub mod gui_lol;
pub mod support;
pub mod lol;
pub mod lol_structs;
pub mod empatica;
pub mod db_structures;


/*
extern crate time;
extern crate rand;
extern crate ordered_float;
extern crate nix;

use std::io;
use std::process::Command;
use std::str::FromStr;
use std::string::ParseError;
use std::thread;
use std::sync::mpsc;
use std::fs::File;

use std::io::BufReader;
use rand::Rng;
use std::cmp::Ordering;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use std::fs;

*/