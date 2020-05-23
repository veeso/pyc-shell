//! ## AsyncStdin
//!
//! `AsyncStdin` module implements two different methods to use an asynchronous stdin

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "Pyc"
*
*   Pyc is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   Pyc is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with Pyc.  If not, see <http://www.gnu.org/licenses/>.
*
*/

use std::io;
use std::os::unix::io::RawFd;

const STDIN_FILENO: RawFd = 0;

/// ### is_ready
/// 
/// Returns whether stdin is ready to be read
pub fn is_ready() -> bool {
    let mut poll_fds: [nix::poll::PollFd; 1] = [nix::poll::PollFd::new(STDIN_FILENO, nix::poll::PollFlags::POLLIN | nix::poll::PollFlags::POLLRDBAND | nix::poll::PollFlags::POLLHUP)];
    match nix::poll::poll(&mut poll_fds, 10) {
        Ok(ret) => {
            if ret > 0 && poll_fds[0].revents().is_some() { //Stdin is available to be read
                let event: nix::poll::PollFlags = poll_fds[0].revents().unwrap();
                if event.intersects(nix::poll::PollFlags::POLLIN) || event.intersects(nix::poll::PollFlags::POLLRDBAND) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        },
        Err(_) => false
    }
}

/// ### read
/// 
/// Read from stdin
pub fn read() -> String {
    let mut stdin: String = String::new();
    let _ = io::stdin().read_line(&mut stdin);
    stdin
}

//@! Test module (Does not work)

/*
#[cfg(test)]
mod tests {

    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_utils_async_stdin() {
        //Should not be ready
        assert_eq!(is_ready(), false);
        //Write to stdin
        assert!(write_to_stdin(String::from("INPUT\n")));
        sleep(Duration::from_millis(100));
        //Now stdin should be ready
        assert_eq!(is_ready(), true);
        //Read
        assert_eq!(read(), String::from("INPUT\n"));
    }

    fn write_to_stdin(data: String) -> bool {
        //Write data out (2048 or remaining bytes)
        let data_out = data.as_bytes();
        let total_bytes_amount: usize = data_out.len();
        let mut bytes_written: usize = 0;
        while bytes_written < total_bytes_amount {
            let bytes_out = if total_bytes_amount - bytes_written > 2048 {
                2048
            } else {
                total_bytes_amount - bytes_written
            };
            match nix::unistd::write(STDIN_FILENO, &data_out[bytes_written..(bytes_written + bytes_out)]) {
                Ok(bytes) => {
                    bytes_written += bytes; //Increment bytes written of bytes
                },
                Err(_) => {
                    return false
                }
            }
        }
        true
    }
}
*/
