cargo build --release

rm ~/.local/bin/rust-todo
ln -s $PWD/target/release/rust-todo ~/.local/bin/
