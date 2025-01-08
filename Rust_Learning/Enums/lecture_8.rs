enum OperatingSystem {
    Linux,
    MacOS,
    Windows,
}

fn main() {
    let my_os = OperatingSystem::Linux;
    let age = which_os(my_os);
    println!("Linux is {age} years old.");

    let friends_os = OperatingSystem::Windows;
    let age = which_os(friends_os);
    println!("{age}");
}

fn which_os(os: OperatingSystem) -> u32 {
    match os {
        OperatingSystem::Linux => 34,
        OperatingSystem::MacOS => 23,
        OperatingSystem::Windows => {
            println!("Windows is really annoying.");
            39
        }
    }
}
