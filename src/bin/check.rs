use std::{error::Error, path::PathBuf};

use itertools::Itertools;
use pdf::file::File as PdfFile;
use structopt::StructOpt;

use typed_pdf::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "Portfolio Planner")]
struct Opt {
    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    for filename in &opt.files {
        if let Some(filename) = filename.to_str() {
            println!("File: {}", filename);
        } else {
            println!("Illegal filename");
        }
        let file = PdfFile::open(filename)?;

        for page in file.pages() {
            let page = page?;
            if let Some(contents) = &page.contents {
                for operation in &contents.operations {
                    if let Operation::Unknown { operator, operands } =
                        normalize_operation(operation)
                    {
                        println!(
                            "Unknown Operation {} {}",
                            operator,
                            operands.iter().map(|op| format!("{:?}", op)).format(", ")
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
