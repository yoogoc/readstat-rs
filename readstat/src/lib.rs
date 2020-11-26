#![allow(non_camel_case_types)]

mod cb;
mod rs;
// mod util;

use colored::Colorize;
use dunce;
use log::debug;
use path_clean::PathClean;
use readstat_sys;
use std::env;
use std::error::Error;
use std::ffi::CString;
use std::path::PathBuf;
use structopt::clap::arg_enum;
use structopt::StructOpt;
// use util::validate_path;

pub use rs::{ReadStatData, ReadStatMetadata, ReadStatVar, ReadStatVarMetadata, ReadStatVarTrunc};

// StructOpt
#[derive(StructOpt, Debug)]
#[structopt(about = "Utility for sas7bdat files")]
pub struct ReadStat {
    #[structopt(parse(from_os_str))]
    /// Path to sas7bdat file
    in_file: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Get sas7bdat metadata
    Metadata {},
    /// Print rows to standard out
    Preview {
        #[structopt(long, default_value = "10")]
        rows: u32,
    },
    /// Write parsed data to file of specific type
    Data {
        /// Output file path
        #[structopt(long, parse(from_os_str))]
        out_file: Option<PathBuf>,
        /// Output type, defaults to csv
        #[structopt(long, default_value="csv", possible_values=&OutType::variants(), case_insensitive=true)]
        out_type: OutType,
    },
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub enum OutType {
        csv,
    }
}

pub struct ReadStatPath {
    pub path: PathBuf,
    pub extension: String,
    pub cstring_path: CString
}

impl ReadStatPath {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let p = Self::validate_path(path)?;
        let ext = Self::validate_extension(&p)?;
        let csp = Self::path_to_cstring(&p)?;

        Ok(Self {
            path: p,
            extension: ext,
            cstring_path: csp
        })
    }

    #[cfg(unix)]
    pub fn path_to_cstring(path: &PathBuf) -> Result<CString, Box<dyn Error>> {
        use std::os::unix::ffi::OsStrExt;
        let bytes = path.as_os_str().as_bytes();
        CString::new(bytes).map_err(|_| From::from("Invalid path"))
    }

    #[cfg(not(unix))]
    pub fn path_to_cstring(path: &PathBuf) -> Result<CString, Box<dyn Error>> {
        let rust_str = &self.path
            .as_os_str()
            .as_str()
            .ok_or(Err(From::from("Invalid path")))?;
        // let bytes = &self.path.as_os_str().as_bytes();
        CString::new(rust_str).map_err(|_| From::from("Invalid path"))
    }

    fn validate_path(p: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        let abs_path = if p.is_absolute() {
            p
        } else {
            env::current_dir()?.join(p)
        };
        let abs_path = abs_path.clean();

        if abs_path.exists() {
            Ok(abs_path)
        } else {
            Err(From::from(format!(
                "File {} does not exist!",
                abs_path.to_string_lossy().yellow()
            )))
        }
    }

    fn validate_extension(path: &PathBuf) -> Result<String, Box<dyn Error>> {
        path.extension()
            .and_then(|e| e.to_str())
            .and_then(|e| Some(e.to_owned()))
            .map_or(
                Err(From::from(format!(
                    "File {} does not have an extension!",
                    path.to_string_lossy().yellow()
                ))),
                |e| Ok(e),
            )
    }
}

pub fn run(rs: ReadStat) -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let sas_path = dunce::canonicalize(&rs.in_file)?;
    let sas_path = ReadStatPath::new(sas_path)?;

    debug!(
        "Counting the number of variables within the file {}",
        sas_path.path.to_string_lossy()
    );

    match rs.cmd {
        Command::Metadata {} => {
            let mut md = ReadStatMetadata::new(sas_path);
            let error = md.get_metadata()?;

            if error != readstat_sys::readstat_error_e_READSTAT_OK {
                Err(From::from("Error when attempting to parse sas7bdat"))
            } else {
                println!(
                    "Metadata for the file {}\n",
                    md.path.to_string_lossy().yellow()
                );
                println!("{}: {}", "Row count".green(), md.row_count);
                println!("{}: {}", "Variable count".red(), md.var_count);
                println!("{}:", "Variable names".blue());
                for (k, v) in md.vars.iter() {
                    println!(
                        "{}: {} of type {}",
                        k.var_index,
                        k.var_name.bright_purple(),
                        v
                    );
                }
                Ok(())
            }
        }
        Command::Preview { rows: _ } => {
            let mut md = ReadStatMetadata::new(sas_path);
            let error = md.get_metadata()?;

            if error != readstat_sys::readstat_error_e_READSTAT_OK {
                Err(From::from("Error when attempting to parse sas7bdat"))
            } else {
                // Write header
                for (k, _) in md.vars.iter() {
                    if k.var_index == md.var_count - 1 {
                        println!("{}", k.var_name);
                    } else {
                        print!("{}\t", k.var_name);
                    }
                }
                // Write data to standard out
                let error = md.print_data()?;

                if error != readstat_sys::readstat_error_e_READSTAT_OK {
                    Err(From::from("Error when attempting to parse sas7bdat"))
                } else {
                    Ok(())
                }
            }
        }
        Command::Data { out_file: _, out_type: _ } => {
            let mut md = ReadStatMetadata::new(sas_path);
            let error = md.get_metadata()?;

            if error != readstat_sys::readstat_error_e_READSTAT_OK {
                Err(From::from("Error when attempting to parse sas7bdat"))
            } else {
                // Get data
                let mut d = ReadStatData::new(md);
                let error = d.get_data()?;

                if error != readstat_sys::readstat_error_e_READSTAT_OK {
                    Err(From::from("Error when attempting to parse sas7bdat"))
                } else {
                    let out_dir =
                        dunce::canonicalize(PathBuf::from("/home/calex/code/readstat-rs/data"))
                            .unwrap();
                    let out_file = out_dir.join("cars_serde.csv");
                    println!("out_file is {}", out_file.to_string_lossy());
                    d.write(out_file)?;
                    Ok(())
                }
            }
        }
    }
}
