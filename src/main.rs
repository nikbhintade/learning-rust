mod greet;

// Steps to modularize the code:
// 1. Create a new file (e.g., `greet.rs`) in the `src` directory.
// 2. Move the related function(s) into the new file and add `pub` to make them accessible.
// 3. In `main.rs`, use `mod greet;` to include the file as a module.
// 4. If the file is inside a folder (e.g., `src/utils/`), use `mod utils::greet;` in `main.rs`.
// 5. Use `utils::greet::function_name()` to call the function from the module, or bring it into scope with `use utils::greet::function_name`.
// 6. Call the function using `greet::function_name()` from `main.rs`.


fn main() {
    greet::greet();
}
