/* Casper Neo. MPCS 52010. Computer Architecture. */

extern crate rand;
use std::cmp::{min, max};
use self::rand::{thread_rng, Rng};


use cpu::Cpu;

pub fn store_random_numbers(cpu: &mut Cpu, n:usize) {
  // Fills `cpu` with `n` random f64s. if `reset`, resets cache and cpu counters.
  info!("Storing {:?} random f64s to cpu", n);
  let mut rng = thread_rng();
  for i in 0 .. n {
    cpu.ram[i] = rng.next_f64();
  }
}


pub fn dot(cpu: &mut Cpu, n:usize) {
  // Dots two `n` length arrays.
  for i in 0 .. n {
    let cij = cpu.load(i) * cpu.load(i + n);
    cpu.store(2 * n + i, cij);
  }
}


pub fn mxm(cpu: &mut Cpu, n:usize) {
  // Matrix multiplies two n x n arrays
  for i in 0 .. n {
    for j in 0 .. n {
      for k in 0 .. n {
        let aij = cpu.load(i * n + j);
        let bjk = cpu.load(j * n + k + n * n);
        cpu.store(i * n + k + 2 * n * n, aij * bjk);
      }
    }
  }
}


pub fn mxm_block(cpu: &mut Cpu, n:usize, block_size:usize) {
  // Block matrix multiplication

  // Helper functions to access matrices
  let idx_a = |i,j| i * n + j;
  let idx_b = |i,j| i * n + j + n * n;
  let idx_c = |i,j| i * n + j + n * n * 2;
  let num_blocks = max(n / block_size, 1);

  // For each block
  for si in 0 .. num_blocks {
    for sj in 0 .. num_blocks {
      for sk in 0 .. num_blocks {
        // Do the normal matrix multiply
        for i in si * block_size .. min((si + 1) * block_size, n) {
          for j in sj * block_size .. min((sj + 1) * block_size, n) {

            let mut cij = cpu.load(idx_c(i,j));

            for k in sk * block_size .. min((sk + 1) * block_size, n) {
              cij += cpu.load(idx_a(i,j)) * cpu.load(idx_b(j,k));
            }

            cpu.store(idx_c(i,j), cij);
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use cpu::ReplacementPolicy::LRU;

  #[test]
  fn test_dot_product() {
    let mut cpu = Cpu::new(512, 64, 8, LRU, 128);

    for i in 0..8 {
      cpu.ram[i] = 2.0;
      cpu.ram[i + 8] = 4.0;
    }

    dot(&mut cpu, 8);

    for i in 16 .. 24{
      assert!((cpu.load(i) - 8.0).abs() < 0.0000001, cpu.load(i));
    }
  }

  #[test]
  fn test_mxm() {
    let mut cpu = Cpu::new(512, 64, 8, LRU, 512);
    for i in 0..64 {
      cpu.ram[i] = 2.0;
      cpu.ram[i+ 64] = 3.0;
    }
    mxm(&mut cpu, 8);
    for i in (64 * 2) .. (64 * 3) {
      assert!((cpu.load(i) - 48.0) < 0.0000001, cpu.load(i));
    }
  }

  #[test]
  fn test_mxm_block() {
    let mut cpu = Cpu::new(512, 64, 8, LRU, 512);
    for i in 0..64 {
      cpu.ram[i] = 2.0;
      cpu.ram[i + 64] = 3.0;
    }
    mxm_block(&mut cpu, 8, 4);
    for i in (64 * 2) .. (64 * 3){
      assert!((cpu.load(i) - 48.0) < 0.0000001, cpu.load(i));
    }
  }

}
