/* Adapted from https://github.com/eminence/setenv */

use std::convert::AsRef;
use std::env::var_os;
use std::ffi::OsStr;

/// The types of shells we know about
pub enum Shell {
    Windows,
    /// The default if we can't figure out the shell
    Bash,
    Tcsh,
    Zsh,
    Ksh,
}

/// Figure out what shell we are using.  If we can't figure it out, fallback to `Bash`, since many
/// shells support the same `export foo=bar` syntax from bash.
pub fn get_shell() -> Shell {
    if cfg!(windows) {
        Shell::Windows
    } else {
        if let Some(shell) = var_os("BASH") {
            if shell.to_string_lossy().ends_with("/bash") {
                return Shell::Bash;
            }
        }
        if let Some(zsh) = var_os("ZSH_NAME") {
            if zsh.to_string_lossy() == "zsh" {
                return Shell::Zsh;
            }
        }
        if let Some(shell) = var_os("shell") {
            if shell.to_string_lossy().ends_with("/tcsh") {
                return Shell::Tcsh;
            }
        }
        return match var_os("SHELL") {
            None => Shell::Bash,
            Some(oss) => {
                if oss.to_string_lossy().ends_with("/bash") {
                    Shell::Bash
                } else if oss.to_string_lossy().ends_with("/ksh") {
                    Shell::Ksh
                } else if oss.to_string_lossy().ends_with("/zsh") {
                    Shell::Zsh
                } else if oss.to_string_lossy().ends_with("/tcsh") {
                    Shell::Tcsh
                } else {
                    Shell::Bash
                } // many shells support export foo=bar
            }
        };
    }
}

impl Shell {
    /// Returns the name of this shell
    // pub fn get_name(&self) -> &'static str {
    //     match *self {
    //         Shell::Windows => "Windows",
    //         Shell::Bash => "bash",
    //         Shell::Tcsh => "tcsh",
    //         Shell::Zsh => "zsh",
    //         Shell::Ksh => "ksh"
    //     }
    // }

    /// Prints to stdout the necessary command to change directory.
    // pub fn cd<P: AsRef<OsStr>>(&self, p: P) {
    //     match *self {
    //         Shell::Windows => {
    //             println!("cd /d \"{}\"", p.as_ref().to_string_lossy());
    //         }
    //         _ => {
    //             println!("cd '{}';", p.as_ref().to_string_lossy());
    //         }
    //     }
    // }

    /// Returns the necessary command to set an envionrment variable
    pub fn setenv<K: AsRef<OsStr>, V: AsRef<OsStr>>(&self, k: K, v: V) -> String {
        match *self {
            Shell::Windows => {
                return format!(
                    "set {}={}",
                    k.as_ref().to_string_lossy(),
                    v.as_ref().to_string_lossy()
                )
            }
            Shell::Tcsh => {
                return format!(
                    "setenv {} '{}'",
                    k.as_ref().to_string_lossy(),
                    v.as_ref().to_string_lossy()
                )
            }
            _ => {
                return format!(
                    "export {}='{}'",
                    k.as_ref().to_string_lossy(),
                    v.as_ref().to_string_lossy()
                )
            }
        }
    }

    /// Returns the necessary command to unset a function
    pub fn get_unset_function(&self) -> String {
        match *self {
            Shell::Zsh => String::from("unset -f"),
            _ => String::from("unset -f")
        }
    }

    // /// A simple wrapper around `std::env::split_paths`
    // pub fn split_env<K: AsRef<OsStr>>(&self, k: K) -> Vec<OsString> {
    //     match std::env::var(k) {
    //         Err(..) => Vec::new(),
    //         Ok(ref v) => std::env::split_paths(v).map(|p| p.into_os_string()).collect(),
    //     }
    // }

    // /// A simple wrapper around `std::env::join_paths` and `setenv`
    // pub fn setenv_list<K, I, T>(&self, k: K, v: I)
    //     where K: AsRef<OsStr>,
    //           I: IntoIterator<Item = T>,
    //           T: AsRef<OsStr>
    // {
    //     let paths = std::env::join_paths(v).unwrap();
    //     self.setenv(k, paths);
    // }
}
