project name: rustflix-clap-tutorial

Tutorial about how to use Clap based on video [Rust Command Line Argument Parsing (A Better Way With Clap)](https://www.youtube.com/watch?v=fD9ptABVQbI)

other helpful tutorials
[Parse Rust CLI Args With Clap](https://www.youtube.com/watch?v=Ot3qCA3Iv_8&t=211s)

build and install

- cargo build && cargo install --path .

then run

- rustflix-clap-tutorial user create bob 'bob@ale.net'
  output:
  RustflixArgs { entity_type: User(UserCommand { command: Create(CreateUser { name: "bob", email: "bob@ale.net" }) }) }
