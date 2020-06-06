//! ## UnixSignal
//!
//! `UnixSignal` module provides an API to instantiate nix::sys::signal

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

extern crate nix;

use nix::sys::signal::Signal;

/// ## UnixSignal
///
/// The UnixSignal enums represents the UNIX signals
#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum UnixSignal {
    Sighup = 1,
    Sigint = 2,
    Sigquit = 3,
    Sigill = 4,
    Sigtrap = 5,
    Sigabrt = 6,
    Sigbus = 7,
    Sigfpe = 8,
    Sigkill = 9,
    Sigusr1 = 10,
    Sigsegv = 11,
    Sigusr2 = 12,
    Sigpipe = 13,
    Sigalrm = 14,
    Sigterm = 15,
    Sigstkflt = 16,
    Sigchld = 17,
    Sigcont = 18,
    Sigstop = 19,
    Sigtstp = 20,
    Sigttin = 21,
    Sigttou = 22,
    Sigurg = 23,
    Sigxcpu = 24,
    Sigxfsz = 25,
    Sigvtalrm = 26,
    Sigprof = 27,
    Sigwinch = 28,
    Sigio = 29,
    Sigpwr = 30,
    Sigsys = 31
}

impl UnixSignal {

    /// ### from_u8
    /// 
    /// Convert a u8 to a Unix Signal
    pub fn from_u8(sig: u8) -> Option<UnixSignal> {
        match sig {
            1 => Some(UnixSignal::Sighup),
            2 => Some(UnixSignal::Sigint),
            3 => Some(UnixSignal::Sigquit),
            4 => Some(UnixSignal::Sigill),
            5 => Some(UnixSignal::Sigtrap),
            6 => Some(UnixSignal::Sigabrt),
            7 => Some(UnixSignal::Sigbus),
            8 => Some(UnixSignal::Sigfpe),
            9 => Some(UnixSignal::Sigkill),
            10 => Some(UnixSignal::Sigusr1),
            11 => Some(UnixSignal::Sigsegv),
            12 => Some(UnixSignal::Sigusr2),
            13 => Some(UnixSignal::Sigpipe),
            14 => Some(UnixSignal::Sigalrm),
            15 => Some(UnixSignal::Sigterm),
            16 => Some(UnixSignal::Sigstkflt),
            17 => Some(UnixSignal::Sigchld),
            18 => Some(UnixSignal::Sigcont),
            19 => Some(UnixSignal::Sigstop),
            20 => Some(UnixSignal::Sigtstp),
            21 => Some(UnixSignal::Sigttin),
            22 => Some(UnixSignal::Sigttou),
            23 => Some(UnixSignal::Sigurg),
            24 => Some(UnixSignal::Sigxcpu),
            25 => Some(UnixSignal::Sigxfsz),
            26 => Some(UnixSignal::Sigvtalrm),
            27 => Some(UnixSignal::Sigprof),
            28 => Some(UnixSignal::Sigwinch),
            29 => Some(UnixSignal::Sigio),
            30 => Some(UnixSignal::Sigpwr),
            31 => Some(UnixSignal::Sigsys),
            _ => None
        }
    }

    /// ### to_nix_signal
    /// 
    /// Converts a UnixSignal to a nix::signal
    pub fn to_nix_signal(&self) -> Signal {
        match self {
            UnixSignal::Sigabrt => Signal::SIGABRT,
            UnixSignal::Sigalrm => Signal::SIGALRM,
            UnixSignal::Sigbus => Signal::SIGBUS,
            UnixSignal::Sigchld => Signal::SIGCHLD,
            UnixSignal::Sigcont => Signal::SIGCONT,
            UnixSignal::Sigfpe => Signal::SIGFPE,
            UnixSignal::Sighup => Signal::SIGHUP,
            UnixSignal::Sigill => Signal::SIGILL,
            UnixSignal::Sigint => Signal::SIGINT,
            UnixSignal::Sigio => Signal::SIGIO,
            UnixSignal::Sigkill => Signal::SIGKILL,
            UnixSignal::Sigpipe => Signal::SIGPIPE,
            UnixSignal::Sigprof => Signal::SIGPROF,
            UnixSignal::Sigpwr => Signal::SIGPWR,
            UnixSignal::Sigquit => Signal::SIGQUIT,
            UnixSignal::Sigsegv => Signal::SIGSEGV,
            UnixSignal::Sigstkflt => Signal::SIGSTKFLT,
            UnixSignal::Sigstop => Signal::SIGSTOP,
            UnixSignal::Sigsys => Signal::SIGSYS,
            UnixSignal::Sigterm => Signal::SIGTERM,
            UnixSignal::Sigtrap => Signal::SIGTRAP,
            UnixSignal::Sigtstp => Signal::SIGTSTP,
            UnixSignal::Sigttin => Signal::SIGTTIN,
            UnixSignal::Sigttou => Signal::SIGTTOU,
            UnixSignal::Sigurg => Signal::SIGURG,
            UnixSignal::Sigusr1 => Signal::SIGUSR1,
            UnixSignal::Sigusr2 => Signal::SIGUSR2,
            UnixSignal::Sigvtalrm => Signal::SIGVTALRM,
            UnixSignal::Sigwinch => Signal::SIGWINCH,
            UnixSignal::Sigxcpu => Signal::SIGXCPU,
            UnixSignal::Sigxfsz => Signal::SIGXFSZ
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_unixsignal_from_u8() {
        assert_eq!(UnixSignal::from_u8(1).unwrap(), UnixSignal::Sighup);
        assert_eq!(UnixSignal::from_u8(2).unwrap(), UnixSignal::Sigint);
        assert_eq!(UnixSignal::from_u8(3).unwrap(), UnixSignal::Sigquit);
        assert_eq!(UnixSignal::from_u8(4).unwrap(), UnixSignal::Sigill);
        assert_eq!(UnixSignal::from_u8(5).unwrap(), UnixSignal::Sigtrap);
        assert_eq!(UnixSignal::from_u8(6).unwrap(), UnixSignal::Sigabrt);
        assert_eq!(UnixSignal::from_u8(7).unwrap(), UnixSignal::Sigbus);
        assert_eq!(UnixSignal::from_u8(8).unwrap(), UnixSignal::Sigfpe);
        assert_eq!(UnixSignal::from_u8(9).unwrap(), UnixSignal::Sigkill);
        assert_eq!(UnixSignal::from_u8(10).unwrap(), UnixSignal::Sigusr1);
        assert_eq!(UnixSignal::from_u8(11).unwrap(), UnixSignal::Sigsegv);
        assert_eq!(UnixSignal::from_u8(12).unwrap(), UnixSignal::Sigusr2);
        assert_eq!(UnixSignal::from_u8(13).unwrap(), UnixSignal::Sigpipe);
        assert_eq!(UnixSignal::from_u8(14).unwrap(), UnixSignal::Sigalrm);
        assert_eq!(UnixSignal::from_u8(15).unwrap(), UnixSignal::Sigterm);
        assert_eq!(UnixSignal::from_u8(16).unwrap(), UnixSignal::Sigstkflt);
        assert_eq!(UnixSignal::from_u8(17).unwrap(), UnixSignal::Sigchld);
        assert_eq!(UnixSignal::from_u8(18).unwrap(), UnixSignal::Sigcont);
        assert_eq!(UnixSignal::from_u8(19).unwrap(), UnixSignal::Sigstop);
        assert_eq!(UnixSignal::from_u8(20).unwrap(), UnixSignal::Sigtstp);
        assert_eq!(UnixSignal::from_u8(21).unwrap(), UnixSignal::Sigttin);
        assert_eq!(UnixSignal::from_u8(22).unwrap(), UnixSignal::Sigttou);
        assert_eq!(UnixSignal::from_u8(23).unwrap(), UnixSignal::Sigurg);
        assert_eq!(UnixSignal::from_u8(24).unwrap(), UnixSignal::Sigxcpu);
        assert_eq!(UnixSignal::from_u8(25).unwrap(), UnixSignal::Sigxfsz);
        assert_eq!(UnixSignal::from_u8(26).unwrap(), UnixSignal::Sigvtalrm);
        assert_eq!(UnixSignal::from_u8(27).unwrap(), UnixSignal::Sigprof);
        assert_eq!(UnixSignal::from_u8(28).unwrap(), UnixSignal::Sigwinch);
        assert_eq!(UnixSignal::from_u8(29).unwrap(), UnixSignal::Sigio);
        assert_eq!(UnixSignal::from_u8(30).unwrap(), UnixSignal::Sigpwr);
        assert_eq!(UnixSignal::from_u8(31).unwrap(), UnixSignal::Sigsys);
        assert!(UnixSignal::from_u8(255).is_none());
    }

    #[test]
    fn test_unixsignal_to_nix() {
        assert_eq!(UnixSignal::Sigabrt.to_nix_signal(), Signal::SIGABRT);
        assert_eq!(UnixSignal::Sighup.to_nix_signal(), Signal::SIGHUP);
        assert_eq!(UnixSignal::Sigint.to_nix_signal(), Signal::SIGINT);
        assert_eq!(UnixSignal::Sigquit.to_nix_signal(), Signal::SIGQUIT);
        assert_eq!(UnixSignal::Sigill.to_nix_signal(), Signal::SIGILL);
        assert_eq!(UnixSignal::Sigtrap.to_nix_signal(), Signal::SIGTRAP);
        assert_eq!(UnixSignal::Sigbus.to_nix_signal(), Signal::SIGBUS);
        assert_eq!(UnixSignal::Sigfpe.to_nix_signal(), Signal::SIGFPE);
        assert_eq!(UnixSignal::Sigkill.to_nix_signal(), Signal::SIGKILL);
        assert_eq!(UnixSignal::Sigusr1.to_nix_signal(), Signal::SIGUSR1);
        assert_eq!(UnixSignal::Sigsegv.to_nix_signal(), Signal::SIGSEGV);
        assert_eq!(UnixSignal::Sigusr2.to_nix_signal(), Signal::SIGUSR2);
        assert_eq!(UnixSignal::Sigpipe.to_nix_signal(), Signal::SIGPIPE);
        assert_eq!(UnixSignal::Sigalrm.to_nix_signal(), Signal::SIGALRM);
        assert_eq!(UnixSignal::Sigterm.to_nix_signal(), Signal::SIGTERM);
        assert_eq!(UnixSignal::Sigstkflt.to_nix_signal(), Signal::SIGSTKFLT);
        assert_eq!(UnixSignal::Sigchld.to_nix_signal(), Signal::SIGCHLD);
        assert_eq!(UnixSignal::Sigcont.to_nix_signal(), Signal::SIGCONT);
        assert_eq!(UnixSignal::Sigstop.to_nix_signal(), Signal::SIGSTOP);
        assert_eq!(UnixSignal::Sigtstp.to_nix_signal(), Signal::SIGTSTP);
        assert_eq!(UnixSignal::Sigttin.to_nix_signal(), Signal::SIGTTIN);
        assert_eq!(UnixSignal::Sigttou.to_nix_signal(), Signal::SIGTTOU);
        assert_eq!(UnixSignal::Sigurg.to_nix_signal(), Signal::SIGURG);
        assert_eq!(UnixSignal::Sigxcpu.to_nix_signal(), Signal::SIGXCPU);
        assert_eq!(UnixSignal::Sigxfsz.to_nix_signal(), Signal::SIGXFSZ);
        assert_eq!(UnixSignal::Sigvtalrm.to_nix_signal(), Signal::SIGVTALRM);
        assert_eq!(UnixSignal::Sigprof.to_nix_signal(), Signal::SIGPROF);
        assert_eq!(UnixSignal::Sigwinch.to_nix_signal(), Signal::SIGWINCH);
        assert_eq!(UnixSignal::Sigio.to_nix_signal(), Signal::SIGIO);
        assert_eq!(UnixSignal::Sigpwr.to_nix_signal(), Signal::SIGPWR);
        assert_eq!(UnixSignal::Sigsys.to_nix_signal(), Signal::SIGSYS);
    }
}
