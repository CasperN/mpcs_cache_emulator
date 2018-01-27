# Casper Neo. MPCS 52010 Computer Architecture.

import os
import subprocess
import re

def get_stats(n_asso=1, block_size=8, cache_size=65536, replacement='LRU',
                   test_size=64, algorithm='mxm'):
  """Returns the stats from the cache emulator"""
  ram_size = max(4 * test_size ** 2, 2 ** 10)

  command = f"./target/release/cache_emulator -n {n_asso} -b {block_size} "
  command += f"-c {cache_size} -r {replacement} -a {algorithm} -m {ram_size} "
  command += f"-t {test_size}"

  raw = subprocess.check_output(command.split(' ')).decode("utf-8")

  results = {}
  for stat in ("read hits", "read misses", "write hits", "write misses"):
    x = re.findall(r"{}\t[0-9]+".format(stat), raw)[0]
    x = x.replace(stat,'')
    results[stat] = int(x)

  return results


if __name__ == '__main__':
  pass
