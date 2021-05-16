use structopt::StructOpt;
// use crate::cli_handler::{validate_cli};
mod cli;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(short = "m", long = "input-metric")]
    pub input_metric: String,

    #[structopt(short = "v", long = "input-value")]
    pub input_value: f32,
}

struct DataSize {
    pub b: f32,
    pub kb: f32,
    pub mb: f32,
    pub gb: f32,
    pub tb: f32,
}

impl DataSize {
    pub fn new(cli: Cli) -> DataSize {
        let default: f32 = 1024.0;

        match cli.input_metric.as_ref() {
            "b" => DataSize {
                b: cli.input_value,
                kb: cli.input_value * default.powi(-1),
                mb: cli.input_value * default.powi(-2),
                gb: cli.input_value * default.powi(-3),
                tb: cli.input_value * default.powi(-4),
            },
            "kb" => DataSize {
                b: cli.input_value * default.powi(1),
                kb: cli.input_value,
                mb: cli.input_value * default.powi(-1),
                gb: cli.input_value * default.powi(-2),
                tb: cli.input_value * default.powi(-3),
            },
            "mb" => DataSize {
                b: cli.input_value * default.powi(2),
                kb: cli.input_value * default.powi(1),
                mb: cli.input_value,
                gb: cli.input_value * default.powi(-1),
                tb: cli.input_value * default.powi(-2),
            },
            "gb" => DataSize {
                b: cli.input_value * default.powi(3),
                kb: cli.input_value * default.powi(2),
                mb: cli.input_value * default.powi(1),
                gb: cli.input_value,
                tb: cli.input_value * default.powi(-1),
            },
            "tb" => DataSize {
                b: cli.input_value * default.powi(4),
                kb: cli.input_value * default.powi(3),
                mb: cli.input_value * default.powi(2),
                gb: cli.input_value * default.powi(1),
                tb: cli.input_value,
            },
            _ => DataSize {
                b: cli.input_value,
                kb: cli.input_value * default.powi(-1),
                mb: cli.input_value * default.powi(-2),
                gb: cli.input_value * default.powi(-3),
                tb: cli.input_value * default.powi(-4),
            },
        }
    }
}

pub fn valid_cli(cli: &Cli) -> bool {
    let valid_metric = ["b", "kb", "mb", "gb", "tb"];
    if valid_metric.contains(&cli.input_metric.as_str()) {
        true
    } else {
        false
    }
}

fn main() {
    let args = Cli::from_args();
    if valid_cli(&args) {
        let data: DataSize = DataSize::new(args);
        println!("Byte: {byte} \nKiloByte: {kilobyte} \nMegaByte: {megabyte} \nGigaByte: {gigabyte} \nTerraByte: {terrabyte} \n "
        , byte = data.b, kilobyte = data.kb,megabyte = data.mb,gigabyte = data.gb,terrabyte = data.tb);
    } else {
        println!("Wrong format")
    }
}
