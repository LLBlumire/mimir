use mimir::StackFrame;

pub fn print(frame: Vec<StackFrame>) -> Vec<StackFrame> {
    println!("{:#?}", frame.last().unwrap().back());
    frame
}
