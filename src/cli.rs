//
// pub mod datasize {
//     pub struct DataSize {
//         pub b: f32,
//         pub kb: f32,
//         pub mb: f32,
//         pub gb: f32,
//         pub tb: f32,
//     }
//
//     impl DataSize {
//         pub fn new(cli: Cli) -> DataSize {
//             let default: f32 = 1024.0;
//
//             match cli.input_metric.as_ref() {
//                 "b" => DataSize {
//                     b: cli.input_value,
//                     kb: cli.input_value * default.powi(-1),
//                     mb: cli.input_value * default.powi(-2),
//                     gb: cli.input_value * default.powi(-3),
//                     tb: cli.input_value * default.powi(-4),
//                 },
//                 "kb" => DataSize {
//                     b: cli.input_value * default.powi(1),
//                     kb: cli.input_value,
//                     mb: cli.input_value * default.powi(-1),
//                     gb: cli.input_value * default.powi(-2),
//                     tb: cli.input_value * default.powi(-3),
//                 },
//                 "mb" => DataSize {
//                     b: cli.input_value * default.powi(2),
//                     kb: cli.input_value * default.powi(1),
//                     mb: cli.input_value,
//                     gb: cli.input_value * default.powi(-1),
//                     tb: cli.input_value * default.powi(-2),
//                 },
//                 "gb" => DataSize {
//                     b: cli.input_value * default.powi(3),
//                     kb: cli.input_value * default.powi(2),
//                     mb: cli.input_value * default.powi(1),
//                     gb: cli.input_value,
//                     tb: cli.input_value * default.powi(-1),
//                 },
//                 "tb" => DataSize {
//                     b: cli.input_value * default.powi(4),
//                     kb: cli.input_value * default.powi(3),
//                     mb: cli.input_value * default.powi(2),
//                     gb: cli.input_value * default.powi(1),
//                     tb: cli.input_value,
//                 },
//                 _ => DataSize {
//                     b: cli.input_value,
//                     kb: cli.input_value * default.powi(-1),
//                     mb: cli.input_value * default.powi(-2),
//                     gb: cli.input_value * default.powi(-3),
//                     tb: cli.input_value * default.powi(-4),
//                 },
//             }
//         }
//     }
// }
