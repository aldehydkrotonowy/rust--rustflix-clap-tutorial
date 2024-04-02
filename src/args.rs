// args.rs file
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct RustflixArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// create, update, delete or show user
    User(UserCommand),
    // /// create, update, delete or show video
    // Video(VideoCommand),
    // /// create, update, delete or show view
    // View(ViewCommand)
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}
#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// create new user
    Create(CreateUser),
    /// update existing user
    Update(UpdateUser),
    /// delete user
    Delete(DeleteUser),
    /// show all users
    Show,
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// name of the user
    pub name: String,
    /// email addres of the user
    pub email: String,
}

#[derive(Debug, Args)]
pub struct UpdateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Args)]
pub struct DeleteUser {
    pub name: String,
}
