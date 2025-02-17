use chrono::Local;
use clap::{Parser, ValueEnum};
use env_logger::Builder;
use image::RgbImage;
use log::info;
use log::LevelFilter;
use std::io::Write;

mod algorithm;
mod error;
mod image_generator;
use algorithm::*;
use error::Error;
use image_generator::ImageGenerator;

#[derive(ValueEnum, Clone, Debug)]
enum Algorithm {
    Chess,
    Bsp,
}

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl Algorithm {
    pub fn create_image(&self, args: &Args) -> Result<RgbImage, Error> {
        if args.width % args.title_size != 0 {
            return Err(Error::new_argument("width must be a mutiple of title size"));
        }
        if args.height % args.title_size != 0 {
            return Err(Error::new_argument(
                "height must be a mutiple of title size",
            ));
        }

        let mut image_generator = ImageGenerator::new(args.width, args.height, args.title_size);
        image_generator.set_grid(!args.no_grid);

        match *self {
            Algorithm::Chess => {
                let map_generator = ChessMapGenerator::new(
                    image_generator.get_title_width(),
                    image_generator.get_title_height(),
                );
                image_generator.create(map_generator)
            }
            Algorithm::Bsp => {
                let map_generator = BspMapGenerator::new(
                    image_generator.get_title_width(),
                    image_generator.get_title_height(),
                );
                image_generator.create(map_generator)
            }
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// image width
    #[clap(long, value_parser, default_value_t = 1024)]
    width: u32,

    /// image height
    #[clap(long, value_parser, default_value_t = 576)]
    height: u32,

    /// title size
    #[clap(long, value_parser, default_value_t = 8)]
    title_size: u32,

    /// algorithm
    #[clap(long, value_parser, default_value_t = Algorithm::Chess)]
    algorithm: Algorithm,

    /// disable grid on output
    #[clap(long, value_parser)]
    no_grid: bool,

    /// Output file
    #[clap(long, value_parser, default_value_t = String::from("output.png"))]
    output: String,
}

fn main() -> Result<(), Error> {
    //
    // cli arg
    let args = Args::parse();

    //
    // logger
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    info!("output file: {}", args.output);
    info!("image width: {}", args.width);
    info!("image height: {}", args.height);
    info!("title size: {}", args.title_size);

    let buffer = args.algorithm.create_image(&args)?;
    buffer.save(args.output)?;
    info!("image saved");
    Ok(())
}
