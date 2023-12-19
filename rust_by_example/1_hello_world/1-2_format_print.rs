fn main() {
    /*
    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as print! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint! but a newline is appended.
     */

    // Positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
}