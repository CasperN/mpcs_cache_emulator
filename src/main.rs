/* Casper Neo. MPCS 52010. Computer Architecture. */

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate pretty_env_logger;
use std::time::SystemTime;

mod cpu;
mod algorithms;

use algorithms::{store_random_numbers, dot, mxm_block, mxm};
use clap::App;
use cpu::Cpu;

fn parse_default<T>(option :Option<&str>, default_value:T) -> T
  where T:std::str::FromStr
{
  if let Some(string) = option {
    if let Ok(res) = string.parse::<T>(){
      return res
    }
  }
  default_value
}

fn main() {
  // Required to start logging
  pretty_env_logger::init();
  // Parse flags
  let yaml = load_yaml!("cli.yml");
  let flags = App::from_yaml(yaml).get_matches();
  // CPU related flags
  let cache_size = parse_default(flags.value_of("cache-size"), 65536);
  let block_size = parse_default(flags.value_of("block-size"), 64);
  let associativity = parse_default(flags.value_of("associativity"), 2);
  let replacement = match flags.value_of("replacement").unwrap_or("LRU") {
    "FIFO"   => cpu::ReplacementPolicy::FIFO,
    "random" => cpu::ReplacementPolicy::Random,
    "LRU"    => cpu::ReplacementPolicy::LRU,
    _ => unreachable!()
  };
  // algorithm related flags
  let algorithm = flags.value_of("algorithm").unwrap_or("mxm");
  let test_size = parse_default(flags.value_of("test-size"), 500);
  let ram_size = (3 * test_size * test_size as usize).next_power_of_two();

  info!("CPU Parameters:   cache_size\t{:?}", cache_size);
  info!("CPU Parameters:   block_size\t{:?}", block_size);
  info!("CPU Parameters:   ram_size\t{:?}", ram_size);
  info!("CPU Parameters:   associativity\t{:?}", associativity);
  info!("CPU Parameters:   replacement\t{:?}", replacement);
  info!("Test Parameters:  algorithm\t{:?}", algorithm);
  info!("Test Parameters:  test_size\t{:?}", test_size);

  let mut cpu = Cpu::new(cache_size, block_size, associativity, replacement, ram_size);
  info!("Cpu Attributes:   words / block\t{:?}", cpu.words);
  info!("Cpu Attributes:   cache_lines\t{:?}", cpu.cache_lines);

  info!("Storing random numbers...");
  store_random_numbers(&mut cpu, 2 * test_size * test_size);

  info!("Emulating...");
  let start = SystemTime::now();
  match algorithm {
    "dot" => dot(&mut cpu, test_size * test_size),
    "mxm" => mxm(&mut cpu, test_size),
    "mxm-block" => mxm_block(&mut cpu, test_size, 8),
    _ => unreachable!()
  };
  println!("\nTime Elapsed {:?}", start.elapsed().unwrap());
  println!("Results:");
  println!("  read hits\t{:?}\n  read misses\t{:?}\n  write hits\t{:?}\n  write misses\t{:?}\n  instruction count\t{:?}\n",
    cpu.read_hits, cpu.read_misses, cpu.write_hits, cpu.write_misses, cpu.instruction_number );

}
