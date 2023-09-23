#! /bin/zsh

# Change to the simpy directory and activate its Python environment
cd simpy || { echo "Failed to cd into simpy. Exiting."; exit 1; }

# Get the path to the virtual environment
venv_path=$(poetry env info --path) || { echo "Failed to get Poetry environment. Exiting."; exit 1; }

# Print and activate the virtual environment
echo "Virtual Environment Path: $venv_path"
source "${venv_path}/bin/activate" || { echo "Failed to activate virtual environment. Exiting."; exit 1; }

# Go back to the original directory
cd .. || { echo "Failed to cd to parent directory. Exiting."; exit 1; }

# Run cargo watch with maturin develop
cargo watch --ignore "simpy/notebooks" -s "cd simpy; maturin develop --profile release" || { echo "Cargo watch failed. Exiting."; exit 1; }
