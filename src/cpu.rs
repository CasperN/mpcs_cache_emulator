/* Casper Neo. MPCS 52010. Computer Architecture. */

extern crate rand;
use self::rand::Rng;


#[derive(Debug)]
pub enum ReplacementPolicy {
  Random,
  LRU, // Least Recently Used
  FIFO, // First In First Out
}

#[derive(Debug, Clone)]
struct CacheLine {
  tag: usize,
  write_time: u64,
  last_used: u64,
  data: Box<Vec<f64>>
}

enum CacheResult {
  Hit(usize),
  Miss(usize)
}
use self::CacheResult::{Hit, Miss};

#[derive(Debug)]
pub struct Cpu{
  // Parameters
  pub cache_size: usize, // Size of cache in bytes
  pub block_size: usize, // Size of data block in bytes
  pub associativity: usize, // n-way associativity of cache (1 is direct mapped cache)
  pub replacement: ReplacementPolicy,

  // Statistics
  pub read_hits: u64,
  pub read_misses: u64,
  pub write_hits: u64,
  pub write_misses: u64,
  time: u64,

  ram: Box<Vec<f64>>, // Index in `self.ram` is the address
  cache: Box<Vec<CacheLine>>
}

impl Cpu {

  pub fn print_cache(&self){
    println!("{:?}", self.cache );
  }
  pub fn print_ram(&self){
    println!("{:?}", self.ram);
  }

  pub fn new(cache_size: usize, block_size: usize, associativity: usize,
    replacement: ReplacementPolicy, ram_size: usize) -> Cpu {
    /* Builds a new CPU
     */
    let ram = Box::new(vec![0.0;ram_size]);
    let cache = Box::new(vec![
      CacheLine {
        tag:1,
        write_time:0,
        last_used:u64::max_value(),
        data: Box::new(vec![0.0; (block_size / 8)])
      };
      cache_size / block_size
    ]);

    Cpu{cache_size, block_size, associativity, replacement, read_hits:0, read_misses:0,
        write_hits:0, write_misses:0, time:0, ram, cache}
  }

  pub fn parts(&self, address: usize) -> (usize, usize, usize) {
    /* Returns tag, index, and offset numbers from address. */

    let words = self.block_size / 8;
    let offset = address % words ;
    let ram_line = address / words;

    let n_sets = self.cache_size / self.block_size / self.associativity;
    let index = ram_line % n_sets;

    let tag = address / n_sets / words;

    // println!("address: {:?}", address);
    // println!("words:\t {:?}", words);
    // println!("offset:\t {:?}", offset);
    // println!("ram_line:{:?}", ram_line);
    // println!("n_sets:\t {:?}", n_sets);
    // println!("index:\t {:?}", index);
    // println!("tag:\t {:?}\n", tag);
    (tag, index, offset)
  }

  fn get_cache_idx(&self, tag:usize, set:usize) -> CacheResult {
    /* returns Hit(line) if (tag,set) in cache else Miss(replacement_line)
     * the type of self.replacement determines the replacement policy.
     */
    for line in set * self.associativity .. (set + 1) * self.associativity {
      if tag == self.cache[line].tag {
        println!("Hit {:?} at cache line {:?}", (tag,set), line);
        return Hit(line);
      }
    }

    let replacement = match self.replacement {

      ReplacementPolicy::Random => {
        // Replace a random line in associative set
        let r = rand::thread_rng().gen_range(0, self.associativity);
        let random_line = set * self.associativity + r;
        random_line
      },

      ReplacementPolicy::LRU => {
        // Replace least-recently-used line
        let mut lru_time = 0;
        let mut lru_line = 0;

        for line in set * self.associativity .. (set + 1) * self.associativity {
          if self.cache[line].last_used > lru_time {
            lru_time = self.cache[line].last_used;
            lru_line = line;
          }
        }
        lru_line
      },

      ReplacementPolicy::FIFO => {
        // Replace the line which was first written
        let mut first_time = u64::max_value();
        let mut first_line = 0;

        for line in set * self.associativity .. (set + 1) * self.associativity {
          if self.cache[line].write_time < first_time {
            first_time = self.cache[line].write_time;
            first_line = line;
          }
        }
        first_line
      }
    };
    println!("Miss {:?} replacement line {:?}", (tag, set), replacement);
    Miss(replacement)
  }

  fn get_ram_line(&self, address:usize) -> Box<Vec<f64>> {
    let words = self.block_size / 8;
    let r = address - address % words;
    Box::new(self.ram[r .. r + words].to_vec())
  }

  pub fn load(&mut self, address: usize) -> f64 {
    // Load from cache if there, else, load from RAM.
    println!("\nLoading address {:?} ", address);

    self.time += 1;
    let (tag, index, offset) = self.parts(address);

    let line = match self.get_cache_idx(tag, index) {
      Hit(line) => {
        self.read_hits += 1;
        line
      }
      Miss(replacement) => {
        self.read_misses += 1;
        self.cache[replacement].data = self.get_ram_line(address);
        self.cache[replacement].tag = tag;
        self.cache[replacement].write_time = self.time;
        replacement
      }
    };
    self.cache[line].last_used = self.time;
    println!("{:?}",self.cache[line]);
    let value = self.cache[line].data[offset];
    // println!("loaded {:?}", value);
    value
  }

  pub fn store(&mut self, address: usize, value: f64) {
    // Store value in Ram and load into cache (Write-through with Write-allocate)
    println!("\nstoring {:?} to {:?}", value, address);

    self.time += 1;
    let (tag, index, offset) = self.parts(address);

    let line = match self.get_cache_idx(tag, index) {
      Hit(line) => {
        self.write_hits += 1;
        line
      },
      Miss(replacement) => {
        self.write_misses += 1;
        self.cache[replacement].data = self.get_ram_line(address);
        self.cache[replacement].tag = tag;
        replacement
      }
    };

    self.ram[address] = value; // Write through
    self.cache[line].data[offset] = value;
    self.cache[line].write_time = self.time;
    self.cache[line].last_used = self.time;
  }
}

/********** PRIVATE FUNCTION TESTS **********/

#[test]
fn test_addressing_fully_associative(){
  // 1 Cache set, 8 Cache lines, 8 words per line
  let cpu = Cpu::new(512, 64, 8, ReplacementPolicy::LRU, 1000000);
  assert_eq!(cpu.parts(0), (0,0,0));
  assert_eq!(cpu.parts(1), (0,0,1));
  assert_eq!(cpu.parts(7), (0,0,7));
  assert_eq!(cpu.parts(8), (1,0,0));
  assert_eq!(cpu.parts(16), (2,0,0));
  assert_eq!(cpu.parts(17), (2,0,1));
  assert_eq!(cpu.parts(19), (2,0,3));
}

#[test]
fn test_addressing_direct_mapping(){
  // 8 Cache Lines, 8 Cache Sets, 8 words per line
  let cpu = Cpu::new(512, 64, 1, ReplacementPolicy::LRU, 1000000);
  assert_eq!(cpu.parts(0), (0,0,0));
  assert_eq!(cpu.parts(1), (0,0,1));
  assert_eq!(cpu.parts(7), (0,0,7));
  assert_eq!(cpu.parts(8), (0,1,0));
  assert_eq!(cpu.parts(10), (0,1,2));
  assert_eq!(cpu.parts(16), (0,2,0));
  assert_eq!(cpu.parts(64), (1,0,0));
  assert_eq!(cpu.parts(75), (1,1,3));
}

#[test]
fn test_addressing_2_associative(){
  // 8 Cache Lines, 4 Cache Sets, 8 words per line
  let cpu = Cpu::new(512, 64, 2, ReplacementPolicy::LRU, 1000000);
  assert_eq!(cpu.parts(0), (0,0,0));
  assert_eq!(cpu.parts(7), (0,0,7));
  assert_eq!(cpu.parts(9), (0,1,1));
  assert_eq!(cpu.parts(17), (0,2,1));
  assert_eq!(cpu.parts(50), (1,2,2));
  assert_eq!(cpu.parts(100), (3,0,4));
}
