use xdg_home::home_dir;

pub fn initialize() {
    let err_msg = "Unable to find your HOME directory. \
        Consider setting $HOME to your home directory and try again.";

    let config_path = home_dir().expect(err_msg);

    println!("{}",config_path.to_str().expect("bad unicode"));
}
