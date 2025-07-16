use clap::Parser;
use rcflib::rcf::{RCFBuilder, RCFOptionsBuilder, RCF};
use std::error::Error;
use std::io;
use std::process;

/// Compute scalar anomaly scores from the input rows and append them to the output rows
#[derive(Parser, Debug)]
#[command(name = "rcf", version)]
struct Args {
    /// The character used as a field delimiter
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    /// Pass if the data contains a header row
    #[arg(long, default_value_t = false)]
    header_row: bool,

    /// Number of trees to use in the forest
    #[arg(short, long, default_value_t = 100, value_name = "TREES")]
    number_of_trees: usize,

    /// Random seed to use
    #[arg(short, long, default_value_t = 42)]
    random_seed: u64,

    /// Points to keep in sample for each tree
    #[arg(short, long, default_value_t = 256)]
    sample_size: usize,

    /// Use cyclic shingles instead of linear shingles
    #[arg(short = 'c', long, default_value_t = false)]
    shingle_cyclic: bool,

    /// Shingle size to use
    #[arg(short = 'g', long, default_value_t = 1)]
    shingle_size: usize,

    /// Window size of the sample or 0 for no window
    #[arg(short, long, default_value_t = 0)]
    window_size: u64,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(args.delimiter as u8)
        .has_headers(args.header_row)
        .from_reader(io::stdin());

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(args.delimiter as u8)
        .from_writer(io::stdout());

    let header = rdr.headers()?;
    let dimensions = header.len();
    let mut forest = create_forest(&args, dimensions);

    for result in rdr.records() {
        let mut record = result?;
        let score = process_line(&mut forest, &record)?;

        record.push_field(&score.to_string());
        wtr.write_record(&record)?;
    }
    wtr.flush()?;

    Ok(())
}

fn create_forest(args: &Args, dimensions: usize) -> Box<dyn RCF> {
    let store_attributes = false;
    let parallel_enabled = false;
    let internal_shingling = false;
    let internal_rotation = false;
    let time_decay = if args.window_size > 0 {
        1.0 / args.window_size as f64
    } else {
        0.0
    };
    let initial_accept_fraction = 1.0;
    let bounding_box_cache_fraction = 1.0;

    RCFBuilder::<u64, u64>::new(dimensions / args.shingle_size, args.shingle_size)
        .tree_capacity(args.sample_size)
        .number_of_trees(args.number_of_trees)
        .random_seed(args.random_seed)
        .store_attributes(store_attributes)
        .parallel_enabled(parallel_enabled)
        .internal_shingling(internal_shingling)
        .internal_rotation(internal_rotation)
        .time_decay(time_decay)
        .initial_accept_fraction(initial_accept_fraction)
        .bounding_box_cache_fraction(bounding_box_cache_fraction)
        .build_default()
        .unwrap()
}

fn process_line(
    forest: &mut Box<dyn RCF>,
    record: &csv::StringRecord,
) -> Result<f64, Box<dyn Error>> {
    let mut point = Vec::new();
    for v in record.iter() {
        point.push(v.parse()?);
    }
    let score = forest.score(&point)?;
    forest.update(&point, 0)?;
    Ok(score)
}
