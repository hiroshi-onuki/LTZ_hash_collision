#!/bin/bash
MAGMA="//Applications/Magma/magma"

{
  echo 'load "count_Fp2types.m";'
  for p in 7 11 13 17 19 23 29 31 37 41 43 47 53 59 61 67 71 73 79 83 89 97; do
    echo "load \"fp2_invariants/fp2_invariants_p${p}.m\";"
    echo "count_Fp2_types(p, inv_list);"
  done
  echo 'quit;'
} | "$MAGMA"