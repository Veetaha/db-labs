use anyhow::Result;
use structopt::{StructOpt, clap::ArgGroup};
use redis_chat::{UserRole, Chat, LoginResult, MessageData};

#[structopt(
    name = "Redis chat",
    about = "A chat console application that interacts with Redis database.",
    author = "Veetaha (github.com/Veetaha)"
)]
#[derive(StructOpt, Debug)]
pub enum Params {
    /// Log in as a user or admin
    LogIn {
        login: String,
        #[structopt(long)]
        is_admin: bool,
    },

    /// Log out
    LogOut {
        login: String
    },

    /// Send a message to the chat
    SendMessage {
        /// User login to send the message on behalf of
        #[structopt(long)]
        sender: String,

        /// Login of the user that should receive the message
        #[structopt(long)]
        receiver: String,

        /// Message text to send
        #[structopt(long)]
        message: String,
    },

    /// View messages sent by the user grouped by status
    SentMessages {
        /// Login of the user for whom to display sent messages
        #[structopt(long)]
        sender: String
    },

    /// View the messages received by the user
    ViewMessages {
        /// Login of the user for whom to view messages
        #[structopt(long)]
        receiver: String,
    },

    /// View the events journal as an admin
    ViewEventsJournal,

    /// View the list of users currently online
    OnlineUsers,

    /// Find users according to the given conditions
    FindUsers(FindUsers)
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("find_user").required(true).multiple(false))]
struct FindUsers {
    /// Show N most chatty users (whose messages did were delievered)
    #[structopt(long, group = "find_user")]
    most_chatty: Option<u32>,

    /// Show N most spammy users (whose messages were marked as spam)
    #[structopt(long, group = "find_user")]
    most_spammy: Option<u32>,
}



pub fn run_ui(params: Params, chat: &mut Chat) -> Result<()> {
    match params {
        Params::LogIn { login, is_admin } => {
            let role = if is_admin { UserRole::Admin } else { UserRole::Regular };

            match chat.login(&login, role)? {
                LoginResult::AlreadyLoggedIn => {
                    println!("{} `{}` is already logged in.", role, login)
                }
                LoginResult::Success => {
                    println!("Successfully logged in as {} {}", role, login)
                }
            }
        }
        Params::LogOut { login } => {
            if chat.logout(&login)? {
                println!("Successfully logged out");
            } else {
                println!("User {} is not online", login);
            }
        }
        Params::SendMessage { sender, receiver, message } => {
            chat.send_message(&MessageData::new(message, sender, receiver))?;
            println!("Message is succesfully sent to processing queue");
        }
        Params::ViewMessages { receiver } => {
            for message in chat.view_messages(&receiver)? {
                println!("{}", message.data);
            }
        }
        Params::ViewEventsJournal => {
            for event in chat.events_stream()? {
                println!("{}", event);
            }
        }
        Params::OnlineUsers => {
            let user_logins = chat.users_online()?;
            println!("Users online ({})", user_logins.len());
            for user_login in user_logins {
                println!("- {}", user_login);
            }
        }
        Params::FindUsers(FindUsers { most_chatty: Some(most_chatty), .. }) => {
            println!("Most chatty users:");
            for stat in chat.top_n_most_chatty_users(most_chatty) {
                println!("- {}", stat);
            }
        }
        Params::FindUsers(FindUsers { most_spammy: Some(most_spammy), .. }) => {
            for stat in chat.top_n_most_spammy_users(most_spammy) {
                println!("- {}", stat);
            }
        }

        Params::SentMessages { sender } => {
            let stat = chat.outcomming_messages_stat(&sender)?;
            println!("{}", stat);
        }
        Params::FindUsers(FindUsers { most_chatty: None, most_spammy: None }) => {
            unreachable!();
        }
    }

    Ok(())
}
