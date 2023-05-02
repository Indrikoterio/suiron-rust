// Set an environment variable for the test dir.
fn main() {
    println!("cargo:rustc-env=SUIRON_TEST_DIR=./tests");
}
