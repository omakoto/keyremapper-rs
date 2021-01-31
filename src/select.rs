/* Copyright (C) 2017 Andrew Ayer
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Except as contained in this notice, the name(s) of the above copyright
 * holders shall not be used in advertising or otherwise to promote the
 * sale, use or other dealings in this Software without prior written
 * authorization.
 */

// From https://gist.github.com/AGWA/b0931a912a8b22b2d6178a3155e171f3

extern crate libc;

use std::{io, ptr, time};
use std::{mem::MaybeUninit, os::unix::io::RawFd};

pub(crate) struct FdSet(libc::fd_set);

impl FdSet {
    pub fn new() -> FdSet {
        unsafe {
            let mut raw_fd_set = MaybeUninit::zeroed().assume_init();
            libc::FD_ZERO(&mut raw_fd_set);
            FdSet(raw_fd_set)
        }
    }
    pub fn clear(&mut self, fd: RawFd) {
        unsafe {
            libc::FD_CLR(fd, &mut self.0);
        }
    }
    pub fn set(&mut self, fd: RawFd) {
        unsafe {
            libc::FD_SET(fd, &mut self.0);
        }
    }
    pub fn is_set(&mut self, fd: RawFd) -> bool {
        unsafe { libc::FD_ISSET(fd, &mut self.0) }
    }
}

pub(crate) fn pselect(
    nfds: libc::c_int,
    readfds: Option<&mut FdSet>,
    writefds: Option<&mut FdSet>,
    errorfds: Option<&mut FdSet>,
    timeout: Option<&libc::timespec>,
    sigmask: Option<&libc::sigset_t>,
) -> io::Result<usize> {
    fn to_fdset_ptr(opt: Option<&mut FdSet>) -> *mut libc::fd_set {
        match opt {
            None => ptr::null_mut(),
            Some(&mut FdSet(ref mut raw_fd_set)) => raw_fd_set,
        }
    }
    fn to_ptr<T>(opt: Option<&T>) -> *const T {
        match opt {
            None => ptr::null::<T>(),
            Some(p) => p,
        }
    }

    match unsafe {
        libc::pselect(
            nfds,
            to_fdset_ptr(readfds),
            to_fdset_ptr(writefds),
            to_fdset_ptr(errorfds),
            to_ptr(timeout),
            to_ptr(sigmask),
        )
    } {
        -1 => Err(io::Error::last_os_error()),
        res => Ok(res as usize),
    }
}

pub(crate) fn make_timespec(duration: time::Duration) -> libc::timespec {
    libc::timespec {
        tv_sec: duration.as_secs() as i64,
        tv_nsec: duration.subsec_nanos() as i64,
    }
}

pub(crate) fn select(fds: &Vec<RawFd>) -> io::Result<RawFd> {
    if fds.len() == 0 {
        panic!("fds can't be empty");
    }
    unsafe {
        loop {
            let mut fd_set = FdSet::new();
            for fd in fds {
                fd_set.set(*fd);
            }
            let max = fds.iter().max().unwrap();

            let mut sigmask: libc::sigset_t = MaybeUninit::zeroed().assume_init();
            libc::sigemptyset(&mut sigmask as *mut libc::sigset_t);

            pselect(max + 1, Some(&mut fd_set), None, None, None, Some(&sigmask))?;
            for i in 0..(max + 1) {
                if fd_set.is_set(i) {
                    return Ok(i);
                }
            }
            eprintln!("No fds selected after pselect()!");
        }
    }
}
