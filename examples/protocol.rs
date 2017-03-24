// Copyright (C) 2016  ParadoxSpiral
//
// This file is part of mpv-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

extern crate mpv;

use mpv::*;
use mpv::protocol::*;

use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem;
use std::slice;
use std::time::Duration;
use std::thread;

fn open(_: &mut (), uri: String) -> File {
    // Open the file, and strip the `filereader://` part
    let ret = File::open(&uri[13..]).unwrap();

    println!("Opened file[{}], ready for orders o7", &uri[13..]);
    ret
}

fn close(_: Box<File>) {
    println!("Closing file, bye bye~~");
}

fn read(cookie: &mut File, buf: *mut i8, nbytes: u64) -> i64 {
    unsafe {
        let slice = slice::from_raw_parts_mut(buf, nbytes as _);
        let forbidden_magic = mem::transmute::<&mut [i8], &mut [u8]>(slice);

        cookie.read(forbidden_magic).unwrap() as _
    }
}

fn seek(cookie: &mut File, offset: i64) -> i64 {
    println!("Seeking to byte {}", offset);
    cookie.seek(SeekFrom::Start(offset as u64)).unwrap() as _
}

fn size(cookie: &mut File) -> i64 {
    cookie.metadata().unwrap().len() as _
}

pub fn main() {
    let path =
        format!("filereader://{}",
                env::args().nth(1).expect("Expected path to local media as argument, found nil."));

    let protocol = unsafe {
        Protocol::new("filereader".into(),
                      (),
                      open,
                      close,
                      read,
                      Some(seek),
                      Some(size))
    };

    let mpv = Parent::with_options(false, &[("volume", 30.into())]).unwrap();
    let proto_ctx = mpv.create_protocol_context(1).unwrap();
    proto_ctx.register(protocol).unwrap();

    mpv.playlist_load_files(&[(&path, FileState::AppendPlay, None)]).unwrap();

    thread::sleep(Duration::from_secs(10));

    mpv.seek_forward(&Duration::from_secs(15)).unwrap();

    thread::sleep(Duration::from_secs(5));
}