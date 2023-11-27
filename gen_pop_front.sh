#!/usr/bin/env bash

gen_tuple() {
  offset=$(test -n $2 && echo "$2" || echo "0")
  for i in $(seq 1 "$1"); do
    idx=$((offset + i))
    r="$r T$idx,"
  done
  echo "$r"
}

gen_ops() {
  echo "("
  for v in $(seq 1 $(($1 - 1))); do
    echo "$2.$v ,"
  done
  echo ")"
}

gen_impl() {
  for i in $(seq 1 16); do
    minus=$(($i - 1))
    echo "
      impl<$(gen_tuple "$i")> PopFront for ($(gen_tuple "$i")){
          type Output = ($(gen_tuple "$minus" 1));
          fn pop_front(self) -> Self::Output {
            $(gen_ops "$i" self)
          }
      }
    "
  done
}

echo "

pub trait PopFront {
    type Output;
    fn pop_front(self) -> Self::Output;
}
"

gen_impl