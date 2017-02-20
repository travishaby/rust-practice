fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="rust strings.",
             subject="This is one way",
             verb="to print");

    println!("My name is {0}, {1} {0}", "Bond", "James");
    // It will even check to make sure the correct number of arguments are
    // used.

    // Create a structure which contains an `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    use std::fmt;
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    // However, custom types such as this structure require more
    // complicated handling. This will not work.
    println!("This struct `{}` won't print...", Structure(3));

    println!("{number:>0width$}", number=1, width=6);


    println!("Pi is approximated as {0:.3}", 3.141592);
}
