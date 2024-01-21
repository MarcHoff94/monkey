use monkey::repl;

fn main() {
    let args = std::env::args();
    repl::start(args);
    // repl::start_interactive();
}
