# go to directory where the actual program is
cd ../

# assembler
cargo run -p assembler -- ./examples/main.bvm ./examples/main

#run the file:
cargo run ./examples/main
