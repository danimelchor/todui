cargo build --release

rm ~/.local/bin/rust-todo
ln -s $PWD/target/release/rust-todo ~/.local/bin/

mkdir -p ~/.config/rust-todo
rm ~/.config/rust-todo/configuration.yml
rm ~/.config/rust-todo/db.json
cp $PWD/configuration_release.yml ~/.config/rust-todo/configuration.yml
cp $PWD/db.json ~/.config/rust-todo/
