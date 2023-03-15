//mod args;
//mod chunk;
//mod chunk_type;
//mod commands;
//mod png;


// Error and Result type aliases are to make it easy to use the ? operator in my own code.
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    todo!()
}