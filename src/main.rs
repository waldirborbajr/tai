mod about;
mod version;

fn main() {
    println!("{}", about::about());
    println!("{}", version::show_version());
}
