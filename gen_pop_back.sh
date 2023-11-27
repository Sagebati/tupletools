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
  echo ")"
}

gen_impl() {
  for i in $(seq 1 16); do
    minus=$(("$i" - 1))
    echo "
      impl<$(gen_tuple "$i")> PopBack for ($(gen_tuple "$i")){
          type Output = ($(gen_tuple $minus));
          fn pop_back(self) -> Self::Output {
            $(gen_ops $minus self)
          }
      }
    "
  done
}

echo "

pub trait PopBack {
    type Output;
    fn pop_back(self) -> Self::Output;
}
"

gen_impl