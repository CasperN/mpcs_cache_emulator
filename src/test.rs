use super::cpu;

#[cfg(test)]




#[test]
fn test_load_store(){
  let mut cpu = cpu::Cpu::new(512, 64, 8, cpu::ReplacementPolicy::LRU, 128);
  for i in 0..100 {
    cpu.store(i,i as f64);
  }

  cpu.print_cache();
  println!("");
  println!("{:?}", cpu.parts(56));
  println!("");
  cpu.print_ram();

  assert_eq!(cpu.load(56), 56.0);
  // for i in 0..100 {
  //   assert_eq!(cpu.load(i), i as f64);
  // }
}

#[test]
fn test_dot_product() {
  let mut cpu = cpu::Cpu::new(512, 64, 8, cpu::ReplacementPolicy::LRU, 128);

  for i in 0..8 {
    cpu.store(i, 2.0);
  }
  for i in 8..16{
    cpu.store(i, 4.0);
  }

  for i in 0..8 {
    let x = cpu.load(i) * cpu.load(8 + i);
    cpu.store(16 + i, x);
  }
  for i in 16 .. 24{
    assert!((cpu.load(i) - 8.0).abs() < 0.00001, cpu.load(i) );
  }
}
