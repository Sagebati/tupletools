#!/usr/bin/env bash

set -f # remove wildcard

# first param Trait name
# second param function trait name
# third param function operator trait
# fourth is the operator "/" or "+"

trait="
pub trait Division<T: Div + Copy> {
    fn div(self, other: Self) -> Self;
}"

gen_tuple() {
  r="("
  for i in $(seq 1 $1); do
    if test $i -eq $1; then
      r="$r T"
    else
      r="$r T,"
    fi
  done
  r="$r)"
  echo $r
}

gen_ops() {
  echo "("
  for v in $(seq 0 $(($1 - 1))); do
    echo "$2.$v $3 $4.$v ,"
  done
  echo ")"
}

gen_impl() {
  for i in $(seq 2 16); do
    echo "
      impl<T: $3<Output=T> + Copy> $1<T> for $(gen_tuple $i){
          fn $2(self, other: Self) -> Self {
            $(gen_ops $i self $4 other)
          }
      }
    "
  done
}

echo "
use std::ops::$3;

pub trait $1<T> {
    fn $2(self, other: Self) -> Self;
}
"

# test
gen_impl $1 $2 $3 $4
