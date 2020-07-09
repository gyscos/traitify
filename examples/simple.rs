use traitify::traitify;

#[traitify(name = "OrientationT")]
#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

struct Line<O> {
    orientation: O,
}

impl Line<Horizontal> {
    fn horizontal() -> Self {
        Self::new(Horizontal)
    }
}

impl<O: OrientationT> Line<O> {
    fn new(orientation: O) -> Self {
        Self { orientation }
    }
    fn print(&self) {
        self.orientation.with(|o| match o {
            Orientation::Horizontal => println!("-"),
            Orientation::Vertical => println!("|"),
        });
    }
}

fn main() {
    let line = Line::horizontal();
    // Confirming we are zero-sized
    println!("{}", std::mem::size_of_val(&line));
    line.print();
}
