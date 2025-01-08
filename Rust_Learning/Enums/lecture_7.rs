enum OperatingSystem {
    Linux,
    MacOS,
    Windows,
}

fn main() {
    let os = OperatingSystem::Linux;
    which_os(os);
}

fn which_os(os: OperatingSystem) {
    match os {
        OperatingSystem::Linux => println!("I am using LINUX."),
        OperatingSystem::MacOS => println!("say NO to mac"),
        OperatingSystem::Windows => println!("say NO to windows"),
    }
}
