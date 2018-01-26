/* Casper Neo. MPCS 52010. Computer Architecture. */

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
  // Parse flags
  let yaml = load_yaml!("cli.yml");
  let flags = App::from_yaml(yaml).get_matches();
  // CPU related flags
  let cache_size = parse_default(flags.value_of("cache_size"), 65536);
  let block_size = parse_default(flags.value_of("block_size"), 64);
  let ram_size = parse_default(flags.value_of("ram_size"), 1_048_576);
  let associativity = parse_default(flags.value_of("associativity"), 2);
  let replacement = match flags.value_of("replacement").unwrap_or("LRU") {
    "FIFO"   => cpu::ReplacementPolicy::FIFO,
    "random" => cpu::ReplacementPolicy::Random,
    _        => cpu::ReplacementPolicy::LRU
  };
  // algorithm related flags
  let test_size = parse_default(flags.value_of("test_size"), 64);
  let algorithm = flags.value_of("algorithm").unwrap_or("mxm");

  println!("\nCPU Parameters:");
  println!("  cache_size\t{:?}", cache_size);
  println!("  block_size\t{:?}", block_size);
  println!("  ram_size\t{:?}", ram_size);
  println!("  associativity\t{:?}", associativity);
  println!("  replacement\t{:?}\n", replacement);

  println!("Test Parameters:");
  println!("  algorithm\t{:?}", algorithm);
  println!("  test_size\t{:?}", test_size);

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
