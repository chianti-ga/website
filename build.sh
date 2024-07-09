cd backend
cargo build --release
cd ..
cd frontend
trunk build --release
cd ..

mkdir out
cp -R frontend/dist out/dist
cp -R target/release/backend out/

