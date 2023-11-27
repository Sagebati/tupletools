#!/usr/bin/env bash

gen_tuple() {
  for i in $(seq 1 "$1"); do
    r="$r T$i,"
  done
  echo "$r"
}

gen_ops() {
  echo "("
  for v in $(seq 0 $(($1 - 1))); do
    echo "$2.$v ,"
  done
  echo "other"
  echo ")"
}

gen_impl() {
  for i in $(seq 1 16); do
    echo "
      impl<$(gen_tuple "$i") T> Append<T> for ($(gen_tuple "$i")){
          type Output = ($(gen_tuple "$i") T);
          fn append(self, other: T) -> Self::Output {
            $(gen_ops "$i" self)
          }
      }
    "
  done
}

echo "

pub trait Append<T> {
    type Output;
    fn append(self, other: T) -> Self::Output;
}
"

gen_impl