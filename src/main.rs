use structopt::StructOpt;

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

        match cli.input_metric.as_str() {
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

#[derive(Debug, StructOpt)]
#[structopt(name = "simple converter", author = "gao-rangers", about = "An Flinter Example of DataSize Simple Conversion")]
pub struct Cli {
    #[structopt(short = "m", long = "input-metric", default_value = "b")]
    pub input_metric: String,

    #[structopt(short = "v", long = "input-value", default_value = "100")]
    pub input_value: f32,
}

fn main() {
    let args = Cli::from_args();

    if valid_cli(&args) {
        let data: DataSize = DataSize::new(args);
        println!("Byte: {byte} \nKiloByte: {kilobyte} \nMegaByte: {megabyte} \nGigaByte: {gigabyte} \nTerraByte: {terrabyte} \n "
        , byte = data.b, kilobyte = data.kb,megabyte = data.mb,gigabyte = data.gb,terrabyte = data.tb);
    } else {
        eprintln!("Wrong format")
    }
}
