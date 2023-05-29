# Run quickreplace example

# Create test file
echo "Hello World" | out-file hello.txt -encoding utf8

# Run rust code
cargo run "World" "Rust!" hello.txt hello-mod.txt

# Print the diff
diff (get-content foo-mod.txt) (get-content foo.txt)