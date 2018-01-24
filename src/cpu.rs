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

  fn parts(&self, address: usize) -> (usize, usize, usize) {
    /* Returns parts of the tag, index, and offset from address
     */
    let words = self.block_size / 8;
    let offset = address % words ;

    let ram_line = address / words;
    let n_sets = self.cache_size / self.block_size / self.associativity;
    let index = ram_line % n_sets;

    let tag = address / n_sets / words;

    (tag, index, offset)
  }

  fn get_cache(&mut self, address:usize) -> Option<f64> {
    /* Returns Some(Double) if address is in the cache else None.
     */
    let (tag, set, offset) = self.parts(address);

    for line in set * self.associativity .. (set + 1) * self.associativity {
      if tag == self.cache[line].tag {
        self.cache[line].last_used = self.time;
        return Some(self.cache[line].data[offset]);
      }
    }
    None
  }

  fn set_cache(&mut self, address:usize) -> f64 {
    /* Reads in line from self.ram, writes it in cache, then returns the value.
     */
    let (tag, set, _) = self.parts(address);

    // Copy line from Ram
    let ram_line = address - address % self.block_size;
    let new_line = Box::new(self.ram[ram_line .. ram_line + self.block_size / 8].to_vec());

    // Get cache line to replace
    let line = match self.replacement {

      ReplacementPolicy::Random => {
        // Replace a random line in associative set
        let random_line_in_set = rand::thread_rng().gen_range(0, self.associativity);
        set * self.associativity + random_line_in_set
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

    // Replace line
    self.cache[line] = CacheLine{tag, last_used:self.time, write_time:self.time, data: new_line};
    self.ram[address]
  }

  pub fn load(&mut self, address: usize) -> f64 {
    // Load from cache if there, else, load from RAM.
    self.time += 1;

    match self.get_cache(address) {
      Some(data) => {
        self.read_hits += 1;
        data
      }
      None => {
        self.read_misses += 1;
        self.set_cache(address)
      }
    }
  }

  pub fn store(&mut self, address: usize, value: f64) {
    // Store value in Ram and load into cache (Write-through with Write-allocate)
    self.time += 1;

    match self.get_cache(address){
      Some(_) => {self.write_hits += 1},
      None => {self.write_misses += 1}
    };

    self.ram[address] = value;
    self.set_cache(address);
  }

}
