

/* StdOut and StdErr

On most operating systems, a program can write to two output streams, stdout and stderr. stdout is for the program’s actual output, while stderr allows errors and other messages to be kept separate from stdout. That way, output can be stored to a file or piped to another program while errors are shown to the user.

In Rust this is achieved with println! and eprintln!, the former printing to stdout and the latter to stderr.
*/
println!("Hello World");

eprintln!("This is an error! :(");

/* Logging

Logging is the same as using println!, except that you can specify the importance of a message. The levels you can usually use are error, warn, info, debug, and trace (error has the highest priority, trace the lowest).

*/
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
  /*
  To enable different levels of logging when executing a rust project
    > RUST_LOG=<log-level> cargo run
    e.g.
    > RUST_LOG=info cargo run
    or, for any main() function found in an executable file `src/prog.rs`.
    > RUST_LOG=<log-level> cargo run  --bin prog
   */
}



/* Progress bar

Some CLI applications run less than a second, others take minutes or hours. If you are writing one of the latter types of programs, you might want to show the user that something is happening. For this, you should try to print useful status updates, ideally in a form that can be easily consumed.

Using the indicatif crate, you can add progress bars and little spinners to your program. Here’s a quick example:
*/

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

