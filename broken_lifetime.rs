// Python struct with a ref member somewhere in it
struct Python<'a> {
    member : &'a u32,
}

// Participant
struct Participant;

impl Participant {
    pub fn do_something(& mut self){}
}

// Your run function;
fn run<'a>(py : Python<'a>, participant : &'_ mut Participant)
{

    // Do some stuff on another thread, we don't know how long the
    // thread will last so we need participant to last for longer
    rayon::spawn(move || {
        participant.do_something();
    });

}

// 
fn main() {
    let python = Python{ member : &123 };
    let mut participant = Participant{};

    run(python, &mut participant);
}


/*
error[E0621]: explicit lifetime required in the type of `participant`
  --> src/main.rs:21:5
   |
16 | fn run<'a>(py : Python<'a>, participant : &'_ mut Participant)
   |                                           ------------------- help: add explicit lifetime `'static` to the type of `participant`: `&'static mut Participant`
...
21 |     rayon::spawn(move || {
   |     ^^^^^^^^^^^^ lifetime `'static` required

error: aborting due to previous error

For more information about this error, try `rustc --explain E0621`.
error: could not compile `duranti`

To learn more, run the command again with --verbose.
*/
