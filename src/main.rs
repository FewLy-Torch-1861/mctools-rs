use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Simple misc minecraft tools.", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
  /// Convert Overworld coordinates to Nether coordinates.
  #[command(allow_hyphen_values = true)]
  NetherCoord {
    #[arg(short, long, help = "Convert nether to overworld instead.")]
    reverse: bool,

    #[arg(help = "X position")]
    x: i32,

    #[arg(help = "Z position")]
    z: i32,
  },
}

fn main() {
  let cli: Cli = Cli::parse();

  match cli.command {
    Commands::NetherCoord { reverse, x, z } => {
      if reverse {
        convert_to_overworld(x, z);
      } else {
        convert_to_nether(x, z);
      }
    }
  }
}

fn convert_to_nether(x: i32, z: i32) {
  let x_nether: f64 = (x as f64) / 8.0;
  let z_nether: f64 = (z as f64) / 8.0;

  println!("xz: {}, {}", x_nether, z_nether);
}

fn convert_to_overworld(x: i32, z: i32) {
  let x_ow: f64 = (x as f64) * 8.0;
  let z_ow: f64 = (z as f64) * 8.0;

  println!("xz: {}, {}", x_ow, z_ow);
}
