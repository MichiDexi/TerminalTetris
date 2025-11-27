# 32-bit
cargo b --release --target i686-pc-windows-gnu
# cargo b --release --target i686-pc-windows-msvc
# cargo b --release --target aarch64-apple-darwin # Not 32-bit, non-intel version (apple silicon)
cargo b --release --target i686-unknown-linux-gnu

# 64-bit
cargo b --release --target x86_64-pc-windows-gnu
# cargo b --release --target x86_64-pc-windows-msvc
# cargo b --release --target x86_64-apple-darwin
cargo b --release --target x86_64-unknown-linux-gnu

# Move all binaries into a folder
rm -rf build # clear all binaries
mkdir build

mv target/i686-pc-windows-gnu/release/tetris build/tetris-win_gnu-32bit
# mv target/i686-pc-windows-msvc/release/tetris build/tetris-win_msvc-32bit
# mv target/aarch64-apple-darwin/release/tetris build/tetris-macos_non-intel
mv target/i686-unknown-linux-gnu/release/tetris build/tetris-linux-32bit

mv target/x86_64-pc-windows-gnu/release/tetris build/tetris-win_gnu-64bit
# mv target/x86_64-pc-windows-msvc/release/tetris build/tetris-win_msvc-64bit
# mv target/x86_64-apple-darwin/release/tetris build/tetris-macos_intel
mv target/x86_64-unknown-linux-gnu/release/tetris build/tetris-linux-64bit
