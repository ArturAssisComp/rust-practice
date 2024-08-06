use log::debug;
use user_auth::User;

fn main() {
    std::env::set_var("RUST_LOG", "user_auth=info,env_logger_demo=debug");
    env_logger::init();

    debug!("env logger demo started");

    let user = User::new("Artur", "pass123");
    user.sign_in("incorrect password");
    user.sign_in("pass123");
}
