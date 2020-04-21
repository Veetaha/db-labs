# Download console ui
curl https://github.com/Veetaha/db-labs/releases/download/v0.1/console_ui -L --output console_ui
sudo chmod +x console_ui

# Download worker
curl https://github.com/Veetaha/db-labs/releases/download/v0.1/worker -L --output worker
sudo chmod +x worker
./worker
./console_ui




# ------------------

# Compile from sources:

# # Install rustup
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# # Install cargo and rustc
# rustup toolchain install

# # Run the app
# cargo run -p console_ui -- help
