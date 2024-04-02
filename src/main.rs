mod args;
use args::*;
use clap::Parser;

fn handle_show_user(name: &String, email: &String) {
    println!("hey {:?} {}", name, email)
}
fn main() {
    let args: RustflixArgs = RustflixArgs::parse();

    println!("{:?}", args);

    match &args {
        RustflixArgs {
            entity_type:
                EntityType::User(UserCommand {
                    command: UserSubcommand::Create(CreateUser { name, email }),
                }),
        } => handle_show_user(&name, &email),
        _ => println!("others are not imortant"),
    }

    match args {
        RustflixArgs { entity_type } => match entity_type {
            EntityType::User(UserCommand { command }) => match command {
                UserSubcommand::Create(CreateUser { name, email }) => {
                    handle_show_user(&name, &email)
                }
                UserSubcommand::Delete(DeleteUser { name }) => {
                    println!("deleting user with name {}", name)
                }
                UserSubcommand::Update(UpdateUser { name, email }) => {
                    println!("Updating user {} with email {}", name, email)
                }
                UserSubcommand::Show => println!("show user data"),
            },
        },
    }
}
