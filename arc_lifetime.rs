// Python struct with a ref member somewhere in it
struct Python<'a> {
    member : &'a u32,
}

// Participant
struct Participant;

type ParticipantRef = std::sync::Arc<Participant>;

impl Participant {
    pub fn do_something(& mut self){}
}

// Your run function;
fn run<'a>(py : Python<'a>, mut participant_ref : ParticipantRef)
{
    // Do some stuff on another thread, we don't know how long the
    // thread will last so we need participant to last for longer
    rayon::spawn(move || {
        if let Some(participant) = ParticipantRef::get_mut(& mut participant_ref) {
            participant.do_something();
        }
    });
}

// 
fn main() {
    let python = Python{ member : &123 };
    let mut participant = ParticipantRef::new(Participant{}); // Allocate participant on the heap, because it could outlive the lifetime of this stack.

    run(python, participant.clone());
}
