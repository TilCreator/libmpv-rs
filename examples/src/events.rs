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

extern crate crossbeam;

use mpv::*;

use std::process;
use std::path::Path;
use std::time::Duration;
use std::thread;

pub fn exec() {
    let mpv = Parent::new(true).unwrap();
    mpv.set_option(&mut Property::new("cache-initial", Data::new(1))).unwrap();
    mpv.set_option(&mut Property::new("volume", Data::new(20))).unwrap();
    mpv.set_option(&mut Property::new("no-video", Data::new(true))).unwrap();
    mpv.set_option(&mut Property::new("ytdl", Data::new(true))).unwrap();
    mpv.init().unwrap();

    crossbeam::scope(|scope| {
        scope.spawn(|| {
            let iter = mpv.observe_all(&[Event::FileLoaded,
                                         Event::StartFile,
                                         Event::Seek,
                                         Event::PlaybackRestart,
                                         Event::EndFile(None)])
                          .unwrap();

            for elem in iter {
                match elem {
                    Err(Error::NoAssociatedEvent) => {}
                    Err(v) => panic!("unexpected error: {:?}", v),
                    Ok(vec) => {
                        if let Some(&Ok(Event::EndFile(ref v))) = vec.iter().find(|e| {
                            if e.is_ok() {
                                if let &Event::EndFile(_) = e.as_ref().unwrap() {
                                    return true;
                                }
                            }
                            false
                        }) {
                            println!("File ended! Reason: {:?}", v);
                            process::exit(0);
                        } else {
                            println!("1: {:?}", vec);
                        };
                    }
                }
            }
        });
        scope.spawn(|| {
            let iter = mpv.observe_all(&[Event::PropertyChange(Property::new("volume",
                                                                             Data::new(0))),
                                         Event::PropertyChange(Property::new("pause",
                                                                             Data::new(false)))])
                          .unwrap();

            for elem in iter {
                match elem {
                    Err(Error::NoAssociatedEvent) => {}
                    Err(v) => panic!("unexpected error: {:?}", v),
                    Ok(v) => println!("2: {:?}", v),
                }
            }
        });

        mpv.playlist(&PlaylistOp::Loadfiles(&[File::new(Path::new("https://www.youtube.\
                                                                   com/watch?v=DLzxrzFCyOs"),
                                                        FileState::AppendPlay,
                                                        None)]))
           .unwrap();

        thread::sleep(Duration::from_secs(3));
        mpv.set_property(&mut Property::new("volume", Data::new(14))).unwrap();

        thread::sleep(Duration::from_secs(30));

        mpv.playlist(&PlaylistOp::NextForce).unwrap();
    });
}