/* Casper Neo. MPCS 52010. Computer Architecture. */

extern crate rand;
use self::rand::{thread_rng, Rng};

use cpu::Cpu;

pub fn dot(cpu: &mut Cpu, n:usize) {
  // Dots two `n` length arrays.
  let mut rng = thread_rng();
  let mut res = 0.0;

  for i in 0..n {
    // Array a
    cpu.store(i, rng.next_f64());
    // Array b
    cpu.store(i + n, rng.next_f64());
  }
  for i in 0..n {
    res += cpu.load(i) * cpu.load(i + n);
  }
  print!("{:?}", res);
}

pub fn mxm(cpu: &mut Cpu, n:usize) {
  // Matrix multiplies two n x n arrays
  let mut rng = thread_rng();

  for i in 0 .. n * n * 2 {
    cpu.store(i, rng.next_f64());
  }

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

pub fn mxm_block(_cpu: &mut Cpu, _n:usize) {
  // Block matrix multiplies
  unimplemented!()
}
