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

#![deny(missing_docs)]
#![allow(unknown_lints, inline_always)]
#![feature(alloc_system, box_syntax, inclusive_range_syntax, pub_restricted, const_fn, stmt_expr_attributes, type_ascription)]
#![cfg_attr(test, feature(test))]

//! This crate provides abstractions for libmpv of the mpv media player. It is guaranteed
//! that no `Client` or `EventIter` is used after their parent exited.
//!
//! # Examples
//!
//! See the 'examples' directory in the crate root.

extern crate alloc_system;

extern crate encoding;
extern crate libc;
extern crate parking_lot;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_primitive;
#[cfg(test)]
extern crate test;

/// Contains bindings to libmpv functions.
pub mod raw;
pub use raw::MpvLogLevel as LogLevel;
mod wrapper;
pub use wrapper::*;
#[cfg(test)]
mod tests;

#[allow(missing_docs)]
pub const MPV_CLIENT_API_MAJOR: u32 = 1;
#[allow(missing_docs)]
pub const MPV_CLIENT_API_MINOR: u32 = 21;
#[allow(missing_docs)]
pub const MPV_CLIENT_API_VERSION: u32 = mpv_make_version(MPV_CLIENT_API_MAJOR,
                                                         MPV_CLIENT_API_MINOR);
const fn mpv_make_version(major: u32, minor: u32) -> u32 {
    major << 16 | minor
}
