/* Casper Neo. MPCS 52010. Computer Architecture. */

#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate clap;
mod cpu;
mod algorithms;
mod test;

use clap::App;
use cpu::Cpu;
/* TODO
  Flags:
    --cache-size
    --block-size
    --n-assoc
    --replacement
*/

fn parse_default<T>(o :Option<&str>, dv:T) -> T
  where T:std::str::FromStr
{
  if let Some(s) = o {
    if let Ok(res) = s.parse::<T>(){
      return res
    }
  }
  dv
}

fn main() {
  pretty_env_logger::init();
  // Parse flags
  let yaml = load_yaml!("cli.yml");
  let flags = App::from_yaml(yaml).get_matches();
  // CPU related flags
  let cache_size = parse_default(flags.value_of("cache-size"), 65536);
  let block_size = parse_default(flags.value_of("block-size"), 64);
  let ram_size = parse_default(flags.value_of("ram-size"), 1_048_576);
  let associativity = parse_default(flags.value_of("associativity"), 2);
  let replacement = match flags.value_of("replacement").unwrap_or("LRU") {
    "FIFO"   => cpu::ReplacementPolicy::FIFO,
    "random" => cpu::ReplacementPolicy::Random,
    _        => cpu::ReplacementPolicy::LRU
  };
  // algorithm related flags
  let test_size = parse_default(flags.value_of("test-size"), 64);
  let algorithm = flags.value_of("algorithm").unwrap_or("mxm");

  info!("CPU Parameters:   cache_size\t{:?}", cache_size);
  info!("CPU Parameters:   block_size\t{:?}", block_size);
  info!("CPU Parameters:   ram_size\t{:?}", ram_size);
  info!("CPU Parameters:   associativity\t{:?}", associativity);
  info!("CPU Parameters:   replacement\t{:?}", replacement);
  info!("Test Parameters:  algorithm\t{:?}", algorithm);
  info!("Test Parameters:  test_size\t{:?}", test_size);

  let lin_alg_fn = match algorithm {
    "dot" => algorithms::dot,
    "mxm" => algorithms::mxm,
    "mxm-block" => algorithms::mxm_block,
    _ => unimplemented!(),
  };

  let mut cpu = Cpu::new(cache_size, block_size, associativity, replacement, ram_size);
  lin_alg_fn(&mut cpu, test_size);

  println!("\nResults:");
  println!("  read hits\t{:?}\n  read misses\t{:?}\n  write hits\t{:?}\n  write misses\t{:?}\n",
    cpu.read_hits, cpu.read_misses, cpu.write_hits, cpu.write_misses );

}
