# Run your Rust program in the background
target/release/potions-rust &

# Get its PID
PID=$!

# Run sample on the PID
sudo sample $PID

