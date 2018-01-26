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
  pub fn new(cache_size: usize, block_size: usize, associativity: usize,
    replacement: ReplacementPolicy, ram_size: usize) -> Cpu {
    /* Builds a new CPU */

    if block_size % 8 != 0 {
      warn!("block_size {:?} bytes is not multiple of 8.", block_size);
    }
    if ram_size % block_size != 0 {
      warn!("ram_size {:?} not divisible by block_size {:?}.", ram_size, block_size);
    }
    if cache_size % block_size != 0 {
      warn!("cache_size {:?} not divisible by block_size {:?}", cache_size, block_size);
    }
    if (cache_size / block_size) % associativity != 0 {
      warn!("uneven cache lines per cache set.");
    }

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

    (tag, index, offset)
  }

  fn check_cache(&self, tag:usize, index:usize) -> CacheResult {
    /* Checks if the tag and index is in the cache.
     * the type of self.replacement determines the replacement policy.
     */
    for line in index * self.associativity .. (index + 1) * self.associativity {
      if tag == self.cache[line].tag {
        debug!("Hit {:?} at cache line {:?}", (tag,index), line);
        return Hit(line);
      }
    }

    let replacement = match self.replacement {

      ReplacementPolicy::Random => {
        // Replace a random line in associative index
        let r = rand::thread_rng().gen_range(0, self.associativity);
        let random_line = index * self.associativity + r;
        random_line
      },

      ReplacementPolicy::LRU => {
        // Replace least-recently-used line
        let mut lru_time = 0;
        let mut lru_line = 0;

        for line in index * self.associativity .. (index + 1) * self.associativity {
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

        for line in index * self.associativity .. (index + 1) * self.associativity {
          if self.cache[line].write_time < first_time {
            first_time = self.cache[line].write_time;
            first_line = line;
          }
        }
        first_line
      }
    };
    debug!("Miss {:?} replacement line {:?}", (tag, index), replacement);
    Miss(replacement)
  }

  fn load_cache_idx(&mut self, address:usize) -> (usize, usize) {
    /* Handles cache misses and returns a line and offset thats loaded with the address */
    let (tag, index, offset) = self.parts(address);
    let line = match self.check_cache(tag, index) {
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
    (line, offset)
  }

  fn get_ram_line(&self, address:usize) -> Box<Vec<f64>> {
    let words = self.block_size / 8;
    let r = address - address % words;
    Box::new(self.ram[r .. r + words].to_vec())
  }

  pub fn load(&mut self, address: usize) -> f64 {
    // Load from cache if there, else, load from RAM.
    debug!("\nLoading address {:?} ", address);
    self.time += 1;
    let (line, offset) = self.load_cache_idx(address);
    self.cache[line].data[offset]
  }

  pub fn store(&mut self, address: usize, value: f64) {
    // Store value in Ram and load into cache (Write-through with Write-allocate)
    debug!("\nstoring {:?} to {:?}", value, address);
    self.time += 1;
    let (line, offset) = self.load_cache_idx(address);
    self.ram[address] = value; // Write through
    self.cache[line].data[offset] = value;
    self.cache[line].write_time = self.time;
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
