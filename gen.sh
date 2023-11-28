bash gen_op.sh Division div Div / >src/ops/div.rs
bash gen_op.sh Subtraction sub Sub - >src/ops/sub.rs
bash gen_op.sh Multiply mul Mul '*' >src/ops/mul.rs
bash gen_op.sh Addition add Add + >src/ops/add.rs
bash gen_append.sh >src/append.rs
bash gen_prepend.sh >src/prepend.rs
bash gen_pop_back.sh >src/pop_back.rs
bash gen_head_tail.sh >src/head_tail.rs
cargo fmt