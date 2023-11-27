#!/usr/bin/env bash

set -f # remove wildcard

# first param Trait name
# second param function trait name
# third param function operator trait
# fourth is the operator "/" or "+"

gen_tuple() {
  for i in $(seq 1 "$1"); do
    deps=$(test -n "$2" && echo $(deps "T$i") || echo "")
    r="$r T$i $deps,"
  done
  echo "$r"
}

gen_ops() {
  echo "("
  for v in $(seq 0 $(($1 - 1))); do
    echo "$2.$v $3 $4.$v ,"
  done
  echo ")"
}

deps() {
  echo  ": $trait_name<Output=$1>"
}

gen_impl() {
  for i in $(seq 1 16); do
    echo "
      impl<$(gen_tuple "$i" yes)> $1 for ($(gen_tuple "$i")){
          type Output = ($(gen_tuple "$i"));
          fn $2(self, other: Self) -> Self::Output {
            $(gen_ops "$i" self "$4" other)
          }
      }
    "
  done
}

echo "
use core::ops::$3;

pub trait $1 {
    type Output;
    fn $2(self, other: Self) -> Self::Output;
}
"

trait_name=$3
# test
gen_impl "$1" "$2" "$3" "$4"
