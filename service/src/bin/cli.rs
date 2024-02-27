use clap::{Args, Parser, Subcommand};
use diesel::prelude::*;
use diesel::{delete, insert_into, RunQueryDsl};
use livekit_mmla_api::auth::models::{NewUser, Role};
use livekit_mmla_api::schema::users::dsl::{users as utable, *};
use livekit_mmla_api::utils::{get_conn_pool, DBPool};

#[derive(Debug, Parser)]
#[command(name = "livekit-mmla")]
#[command(about = "LiveKit MMLA CLI", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Users(UsersArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct UsersArgs {
    #[command(subcommand)]
    command: UserCommands,
}

#[derive(Debug, Subcommand)]
enum UserCommands {
    Add(AddUserOptions),
    Delete {
        #[arg(
            short,
            long,
            value_name = "USERNAME",
            required = true,
            help = "Username of the user"
        )]
        uname_or_email: String,
    },
}

#[derive(Debug, Args)]
struct AddUserOptions {
    #[arg(
        short,
        long,
        value_name = "USERNAME",
        required = true,
        help = "Username of the user"
    )]
    username: String,

    #[arg(
        short,
        long,
        value_name = "EMAIL",
        required = true,
        help = "Email of the user"
    )]
    email: String,

    #[arg(
        short,
        long,
        value_name = "PASSWORD",
        required = true,
        help = "Password of the user"
    )]
    password: String,

    #[arg(
        short = 'a',
        long,
        value_name = "ADMIN",
        help = "Is the user an admin",
        default_value = "false"
    )]
    is_admin: bool,
}

fn add_user(options: AddUserOptions, pool: &mut DBPool) {
    println!("Adding user with options: {:?}", options);
    let mut conn = pool.get().unwrap();

    let new_user = NewUser {
        username: options.username,
        email: options.email,
        password: options.password,
        role: if options.is_admin {
            Role::User
        } else {
            Role::Admin
        },
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    insert_into(utable)
        .values(&new_user)
        .execute(&mut conn)
        .expect("Error inserting user");
}

fn delete_user(name_or_email: &str, pool: &mut DBPool) {
    println!("Deleting user with username/email: {}", name_or_email);
    let mut conn = pool.get().unwrap();

    let count = delete(
        utable.filter(
            username
                .eq(name_or_email)
                .or(email.eq(name_or_email)),
        ),
    )
    .execute(&mut conn)
    .expect("Error deleting user");

    if count == 0 {
        println!("No user found with username/email: {}", name_or_email);
    } else {
        println!(
            "Deleted {} user(s) with username/email: {}",
            count, name_or_email
        );
    }
}

fn main() {
    let cli = CLI::parse();

    let mut pool = get_conn_pool().clone();

    match cli.command {
        Commands::Users(usrs) => match usrs.command {
            UserCommands::Add(options) => {
                add_user(options, &mut pool);
            }
            UserCommands::Delete { uname_or_email } => {
                delete_user(&uname_or_email, &mut pool);
            }
        },
    }
}
