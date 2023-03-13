cargo build --release

rm ~/.local/bin/todo-rs
ln -s $PWD/target/release/todo-rs ~/.local/bin/
