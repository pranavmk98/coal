/*
 * Alias container manager.
 *
 * Author: Pranav Kumar <pmkumar@cmu.edu>
 */

use clap::clap_app;
use clap::AppSettings;
use scan_fmt::scan_fmt;

use dirs;
use regex::Regex;
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

mod lib;

/***********/
/* Globals */
/***********/

/* Directory structure. */
const ROOT_DIR: &str = ".coal/cons";
const ALIAS_FILE: &str = "aliases";
const VALID_CON_REGEX: &str = "[-_.A-Za-z0-9]+";

/* Environment structure. */
const COAL_VAR: &str = "COAL_ACTIVE";
const NO_CON_ACTIVE: &str = "NO CON";

/* Colored asterisks. */
const DEFAULT_ASTERISK: &str = "\\033[39m*\\033[39m";
// const GREEN_ASTERISK: &str = "\\033[32m*\\033[39m";

/********/
/* Util */
/********/

fn err_clap(err: clap::Error) {
    writeln!(io::stderr(), "{}", err).expect("Unable to write");
    std::process::exit(1);
}

/* Print an error message and exit. */
fn error(msg: &str) -> ! {
    println!("echo 'Error: {}'", msg);
    std::process::exit(1);
}

fn get_root_path() -> PathBuf {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(ROOT_DIR);
            return path;
        }

        None => panic!("No home directory detected"),
    }
}

fn get_alias_file(con: &str) -> PathBuf {
    let mut root_dir = get_root_path();
    root_dir.push(con);
    root_dir.push(ALIAS_FILE);
    return root_dir;
}

fn err_check<T, K>(r: Result<T, K>, err: &str) -> T {
    match r {
        Err(_) => {
            error(err);
        }

        Ok(val) => val,
    }
}

fn is_cur_con(con: &str) -> bool {
    match env::var_os(COAL_VAR) {
        Some(cur_con) => cur_con == con,
        None => false,
    }
}

fn add_output_line(output: &mut String, line: &str) {
    output.push_str(&format!("{};", line));
}

/* Must be a fully POSIX-compliant container name. */
fn is_valid_con_name(con: &str) -> bool {
    let r = Regex::new(VALID_CON_REGEX).unwrap();
    return r.is_match(con) && con != NO_CON_ACTIVE;
}

/* Check if container exists. */
fn con_exists(con: &str) -> bool {
    let root_dir = get_root_path();
    let cons = fs::read_dir(root_dir).expect("Unable to read directory");
    for temp in cons {
        let unwrapped = temp
            .expect("Unable to read container")
            .file_name()
            .into_string()
            .unwrap();
        if con == unwrapped {
            return true;
        }
    }
    return false;
}

/* Check if alias exists in current container. */
fn alias_exists(alias: &str) -> bool {
    let con = match env::var_os(COAL_VAR) {
        Some(cur_con) => cur_con,
        None => error(&format!("${} does not exist. Rerun setup.", COAL_VAR))
    }.into_string().unwrap();
    let search_text = format!("alias {}=", alias);

    let alias_file = get_alias_file(&con)
        .into_os_string()
        .into_string()
        .unwrap();
    let f = err_check(File::open(alias_file), "Unable to access aliases");
    let reader = BufReader::new(&f);

    let mut lines = reader.lines().map(|x| x.unwrap());
    return lines.any(|line| line.contains(&search_text));
}

/* Delete alias from file. Return true if successful, false if doesn't exist. */
fn delete_alias_from_file(file: &str, alias: &str) -> bool {
    let search_text = format!("alias {}=", alias);

    let mut f = err_check(File::open(file), "Unable to access aliases");
    let reader = BufReader::new(&f);

    /* Filter out the line containing the alias to delete. */
    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();
    let new_lines: Vec<String> = lines
        .clone()
        .into_iter()
        .filter_map(|line| {
            if !line.contains(&search_text) {
                Some(line)
            } else {
                None
            }
        })
        .collect();

    /* If no line was deleted, it doesn't exist. */
    if new_lines.len() == lines.len() {
        return false;
    }

    f = err_check(File::create(file), "Unable to write to file");

    /* Write filtered lines back. */
    for line in new_lines {
        err_check(f.write(&line.as_bytes()), "Unable to write to file");
        err_check(f.write(b"\n"), "Unable to write to file");
    }

    return true;
}

/* Get command to set COAL_VAR to status. */
fn set_alias_var(output: &mut String, status: &str) {
    let shell = lib::get_shell();
    let cmd: String = shell.setenv(COAL_VAR, status);
    add_output_line(output, &cmd);
}

/* Unalias all commands in a container. */
fn unalias_all(output: &mut String, con: &str) {
    /* Open alias file. */
    let alias_file = get_alias_file(&con);
    let f = err_check(File::open(alias_file), "Unable to access aliases");
    let reader = BufReader::new(&f);

    /* Construct the string to unset the aliases. */
    let unset_aliases: String = reader
        .lines()
        .map(|x| {
            if let Ok((alias, _)) = scan_fmt!(&x.unwrap(), "alias {}={}", String, String) {
                format!("unalias {}", alias)
            } else {
                error("Invalid alias file");
            }
        })
        .collect::<Vec<String>>()
        .join(";");

    /* Add unset aliases to output string. */
    if unset_aliases != "" {
        add_output_line(output, &unset_aliases);
    }
}

/* Alias all commands in a container. */
fn alias_all(output: &mut String, con: &str) {
    let alias_file = get_alias_file(&con);
    let new_f = err_check(File::open(alias_file), "Unable to access aliases");
    let reader = BufReader::new(&new_f);

    /* Construct the string to set the new aliases. */
    let set_aliases: String = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join(";");

    /* Add unset aliases to output string. */
    if set_aliases != "" {
        add_output_line(output, &set_aliases);
    }
}

/*********/
/* Setup */
/*********/

/* Perform any necessary initial setup. */
fn setup(output: &mut String) {
    let root_dir = get_root_path();

    /* Create root directory if necessary. */
    if !Path::exists(&root_dir) {
        err_check(
            fs::create_dir(root_dir),
            "Cannot initialize coal - insufficient permissions?",
        );
    }

    /* Check environment variable. */
    match env::var_os(COAL_VAR) {
        /* Set on first use. */
        None => {
            set_alias_var(output, NO_CON_ACTIVE);
        }
        _ => (),
    }
}

/***************/
/* Subcommands */
/***************/

/* Create a new alias container and switch to it. */
fn new(output: &mut String, con: &str) {
    /* Check if con is valid name. */
    if !is_valid_con_name(con) {
        error(&format!("Not a valid container name. Only numbers, letters, period, underscore, and hyphen allowed."));
    }

    /* Ensure container doesn't exist. */
    if con_exists(con) {
        error(&format!("Container {} already exists.", con));
    }

    let mut root_dir = get_root_path();
    root_dir.push(con);

    /* Create dir for new container. */
    err_check(
        fs::create_dir(&root_dir),
        "Cannot create directory - insufficient permissions?",
    );

    /* Create new files. */
    let mut aliases = root_dir.clone();
    aliases.push(ALIAS_FILE);
    err_check(
        File::create(aliases),
        "Cannot create file - insufficient permissions?",
    );

    load(output, con);
}

/* Delete an alias container. */
fn delete(output: &mut String, con: &str) {
    if !con_exists(&con) {
        error(&format!("No such container: {}", con));
    }

    /* Check environment variable. */
    if is_cur_con(con) {
        /* If deleting current con, unset all aliases and active con. */
        unalias_all(output, con);

        /* Unset active con. */
        set_alias_var(output, NO_CON_ACTIVE);
    }

    /* Delete container directory. */
    let mut root_dir = get_root_path();
    root_dir.push(con);
    fs::remove_dir_all(&root_dir).expect("Unable to delete container");
}

/* Load a new container. */
fn load(output: &mut String, con: &str) {
    /* Ensure container exists. */
    if !con_exists(con) {
        error(&format!("No such container: {}", con));
    }

    /* Unset current aliases if needed. */
    if let Some(cur_con) = env::var_os(COAL_VAR) {
        if cur_con == con {
            error("Container already loaded");
        }

        if cur_con != NO_CON_ACTIVE {
            /* Unset all current aliases. */
            unalias_all(output, &cur_con.into_string().unwrap());
        }
    }

    /* Set active container to new container. */
    set_alias_var(output, &con);

    /* Load aliases of new container. */
    alias_all(output, &con);
}

/* Display the existing containers. */
fn show_all(output: &mut String) {
    let root_dir = get_root_path();
    let cons = fs::read_dir(root_dir).expect("Unable to read directory");

    for dir in cons {
        let con = dir
            .expect("Unable to read container")
            .file_name()
            .into_string()
            .unwrap();
        if is_cur_con(&con) {
            output.push_str(&format!("echo '{}{}';", con, DEFAULT_ASTERISK));
        } else {
            output.push_str(&format!("echo '{}';", con));
        }
    }
}

/* Display all the aliases for a given container. */
fn show_aliases(output: &mut String, con: &str) {
    /* Ensure container exists. */
    if !con_exists(con) {
            error(&format!("No such container: {}", con));
    }

    /* Open alias file. */
    let alias_file = get_alias_file(&con);
    let f = err_check(File::open(alias_file), "Unable to access aliases");
    let reader = BufReader::new(&f);

    /* Construct the string to output the aliases. */
    let aliases: String = reader
        .lines()
        .map(|x| {
            let y = x.unwrap();
            if let Ok((alias, mut cmd)) = scan_fmt!(&y, "alias {}={/.*/}", String, String) {
                cmd = cmd.replace("\"", "\\\"");
                format!("echo \"{} -> {}\"", alias, cmd)
            } else {
                error("Invalid alias file");
            }
        })
        .collect::<Vec<String>>()
        .join(";");

    /* Add unset aliases to output string. */
    if aliases != "" {
        add_output_line(output, &aliases);
    }
}

/* Add a new alias to the current container. */
fn add_alias(output: &mut String, alias: &str, command: &str) {
    let con = match env::var_os(COAL_VAR) {
        Some(cur_con) => {
            if cur_con == NO_CON_ACTIVE {
                error("No alias container active.");
            };
            cur_con
        }
        /* Set on first use. */
        None => error(&format!("${} does not exist. Rerun setup.", COAL_VAR)),
    }
    .into_string()
    .unwrap();

    if alias_exists(alias) {
        error(&format!("Alias {} already exists", alias));
    }

    /* Add new alias to file. */
    let alias_file = get_alias_file(&con);
    let new_alias = format!("alias {}=\"{}\"", alias, command);

    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&alias_file)
        .unwrap();

    err_check(writeln!(f, "{}", &new_alias), "Unable to write alias.");

    /* Set new alias. */
    add_output_line(output, &new_alias);
}

/* Remove an alias from the current container. */
fn remove_alias(output: &mut String, alias: &str) {
    let con = match env::var_os(COAL_VAR) {
        Some(cur_con) => {
            if cur_con == NO_CON_ACTIVE {
                error("No alias container active.");
            };
            cur_con
        }
        /* Set on first use. */
        None => error(&format!("${} does not exist. Rerun setup.", COAL_VAR)),
    }
    .into_string()
    .unwrap();

    if !alias_exists(alias) {
        error(&format!("No such alias: {}", alias));
    }

    let alias_file = get_alias_file(&con)
        .into_os_string()
        .into_string()
        .unwrap();

    /* Delete alias from file if possible. */
    if delete_alias_from_file(&alias_file, alias) {
        let temp = String::from(format!("unalias {}", alias));
        add_output_line(output, &temp);
    } else {
        error("No such alias.");
    }
}

/*********/
/* Main. */
/*********/

fn main() {
    let mut output: String = String::new();
    setup(&mut output);

    let matches = clap_app!(coal =>
        (version: "1.0")
        (author: "Pranav K. <pmkumar@cmu.edu>")
        (about: "Alias container manager")
        (@subcommand new =>
            (about: "Creates new container and switch to it")
            (@arg con_name: +required "Name of the container to create")
        )
        (@subcommand delete =>
            (about: "Deletes existing container")
            (@arg con_name: +required "Name of the container to delete")
        )
        (@subcommand load =>
            (about: "Switches to existing container")
            (@arg con_name: +required "Name of the container to load")
        )
        (@subcommand show =>
            (about: "Displays existing containers (or aliases, if a container is provided)")
            (@arg con_name: "Name of the container to display aliases for")
        )
        (@subcommand add =>
            (about: "Adds alias to current container")
            (@arg alias_name: +required "Name of the alias to add")
            (@arg command:    +required "Command to alias")
        )
        (@subcommand rem =>
            (about: "Removes alias from current container")
            (@arg alias_name: +required "Name of the alias to remove")
        )
    )
    .setting(AppSettings::DisableVersion)
    .setting(AppSettings::VersionlessSubcommands)
    .get_matches_safe()
    .map_err(|e| err_clap(e))
    .expect("Invalid arguments");

    if let Some(matches) = matches.subcommand_matches("new") {
        new(&mut output, matches.value_of("con_name").unwrap());

    } else if let Some(matches) = matches.subcommand_matches("delete") {
        delete(&mut output, matches.value_of("con_name").unwrap());

    } else if let Some(matches) = matches.subcommand_matches("load") {
        load(&mut output, matches.value_of("con_name").unwrap());

    } else if let Some(matches) = matches.subcommand_matches("show") {
        match matches.value_of("con_name") {
            Some(con) => show_aliases(&mut output, con),
            _ => show_all(&mut output)
        }

    } else if let Some(matches) = matches.subcommand_matches("add") {
        let alias = matches.value_of("alias_name").unwrap();
        let command = matches.value_of("command").unwrap();
        add_alias(&mut output, alias, command);

    } else if let Some(matches) = matches.subcommand_matches("rem") {
        let alias = matches.value_of("alias_name").unwrap();
        remove_alias(&mut output, alias);
    }

    println!("{}", output);
}
