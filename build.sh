cargo build --target x86_64-unknown-linux-gnu --release

mkdir build

mkdir build/x86_64-uknown-linux-gnu/

cp target/x86_64-unknown-linux-gnu/release/tic-tac-toe build/x86_64-uknown-linux-gnu/tic-tac-toe

cd build/x86_64-uknown-linux-gnu/

upx tic-tac-toe

zip -9 tic-tac-toe tic-tac-toe

rm tic-tac-toe