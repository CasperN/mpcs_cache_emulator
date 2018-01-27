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
    /* Builds a new CPU */

    if block_size % 8 != 0 {
      error!("block_size {:?} bytes is not multiple of 8.", block_size);
    }
    if ram_size % block_size != 0 {
      error!("ram_size {:?} not divisible by block_size {:?}.", ram_size, block_size);
    }
    if cache_size % block_size != 0 {
      error!("cache_size {:?} not divisible by block_size {:?}", cache_size, block_size);
    }
    if (cache_size / block_size) < associativity {
      error!("associativity {:?} greater than number of cache lines {:?}",
        associativity, (cache_size / block_size));
    }
    if (cache_size / block_size) % associativity != 0 {
      error!("uneven cache lines per cache set.");
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

  fn parts(&self, address: usize) -> (usize, usize, usize) {
    /* Returns tag, index, and offset numbers from address. */

    let words = self.block_size / 8;
    let offset = address % words ;
    let ram_line = address / words;

    let n_sets = self.cache_size / self.block_size / self.associativity;
    let index = ram_line % n_sets;

    let tag = address / n_sets / words;

    (tag, index, offset)
  }

  fn check_cache(&self, tag:usize, index:usize) -> Result<usize,usize> {
    /* Checks if the tag and index is in the cache.
     * the type of self.replacement determines the replacement policy.
     * Returns Ok(line) if the `tag` is in the cache
     * Returns Err(replacement) if `tag` is not in cache
     */
    for line in index * self.associativity .. (index + 1) * self.associativity {
      if tag == self.cache[line].tag {
        debug!("Hit {:?} at cache line {:?}", (tag,index), line);
        return Ok(line);
      }
    }

    let replacement = match self.replacement {

      ReplacementPolicy::Random => {
        // Replace a random line in associative set
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
    Err(replacement)
  }

  fn get_cache(&mut self, address:usize) -> (usize, usize, bool) {
    /* Handles cache misses and returns a line and offset thats loaded with the address. */
    let (tag, index, offset) = self.parts(address);
    let (line, is_hit) = match self.check_cache(tag, index) {
      Ok(line) => { (line, true) }
      Err(replacement) => {
        // Copy ram line into cache
        let words = self.block_size / 8;
        let r = address - address % words;
        for (i, v) in self.ram[r .. r + words].iter().enumerate(){
          self.cache[replacement].data[i] = *v;
        }
        self.cache[replacement].tag = tag;
        self.cache[replacement].write_time = self.time;
        (replacement, false)
      }
    };
    self.cache[line].last_used = self.time;
    (line, offset, is_hit)
  }

  pub fn load(&mut self, address: usize) -> f64 {
    // Load from cache if there, else, load from RAM.
    debug!("Loading address {:?} ", address);
    self.time += 1;
    let (line, offset, is_hit) = self.get_cache(address);
    if is_hit { self.read_hits += 1 } else { self.read_misses += 1 }
    self.cache[line].data[offset]
  }

  pub fn store(&mut self, address: usize, value: f64) {
    // Store value in Ram and load into cache (Write-through with Write-allocate)
    debug!("Storing {:?} to {:?}", value, address);
    self.time += 1;
    self.ram[address] = value; // Write through
    let (line, offset, is_hit) = self.get_cache(address); // write allocate
    if is_hit { self.write_hits += 1 } else { self.write_misses += 1 }
    self.cache[line].data[offset] = value;
    self.cache[line].write_time = self.time;
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_addressing_fully_associative(){
    // 1 Cache set, 8 Cache lines, 8 words per line
    let cpu = Cpu::new(512, 64, 8, ReplacementPolicy::LRU, 2048);
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
    let cpu = Cpu::new(512, 64, 1, ReplacementPolicy::LRU, 2048);
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
    let cpu = Cpu::new(512, 64, 2, ReplacementPolicy::LRU, 2048);
    assert_eq!(cpu.parts(0), (0,0,0));
    assert_eq!(cpu.parts(7), (0,0,7));
    assert_eq!(cpu.parts(9), (0,1,1));
    assert_eq!(cpu.parts(17), (0,2,1));
    assert_eq!(cpu.parts(50), (1,2,2));
    assert_eq!(cpu.parts(100), (3,0,4));
  }

  #[test]
  fn test_load_store(){
    let mut cpu = Cpu::new(512, 64, 8, ReplacementPolicy::LRU, 128);
    for i in 0..100 {
      cpu.store(i,i as f64);
    }

    for i in 0..100 {
      assert_eq!(cpu.load(i), i as f64);
    }
  }
}
