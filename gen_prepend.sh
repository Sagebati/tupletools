#!/usr/bin/env bash

gen_tuple() {
  for i in $(seq 1 "$1"); do
    r="$r T$i,"
  done
  echo "$r"
}

gen_ops() {
  echo "("
  echo "other ,"
  for v in $(seq 0 $(($1 - 1))); do
    echo "$2.$v ,"
  done
  echo ")"
}

gen_impl() {
  for i in $(seq 1 16); do
    echo "
      impl<$(gen_tuple "$i") T> Prepend<T> for ($(gen_tuple "$i")){
          type Output = (T, $(gen_tuple $i));
          fn prepend(self, other: T) -> Self::Output {
            $(gen_ops "$i" self)
          }
      }
    "
  done
}

echo "

pub trait Prepend<T> {
    type Output;
    fn prepend(self, other: T) -> Self::Output;
}
"

gen_impl