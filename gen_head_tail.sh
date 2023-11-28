#!/usr/bin/env sh

gen_tuple() {
  offset=$(test -n $2 && echo "$2" || echo "0")
  for i in $(seq 1 "$1"); do
    idx=$((offset + i))
    r="$r T$idx,"
  done
  echo "$r"
}

gen_ops() {
  echo "($2.0) ,"
  echo "("
  for v in $(seq 1 $(($1 - 1))); do
    echo "$2.$v ,"
  done
  echo ")"
}

gen_impl() {
  for i in $(seq 1 16); do
    minus=$((i - 1))
    echo "
      impl<T, $(gen_tuple "$minus")> HeadTail for (T, $(gen_tuple "$minus")){
          type Head = T;
          type Tail = ($(gen_tuple "$minus"));
          fn head_tail(self) -> (Self::Head, Self::Tail) {
            ($(gen_ops "$i" self))
          }
      }
    "
  done
}

echo "
pub trait HeadTail {
    type Head;
    type Tail;
    fn head_tail(self) -> (Self::Head, Self::Tail);
}
"

gen_impl