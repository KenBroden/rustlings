#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    // ADDED: Resize, Move, Echo, ChangeColor, Quit.
    
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
