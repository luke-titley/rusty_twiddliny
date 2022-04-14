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
