use std::thread;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let sout = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("encryptedfs.out")
        .unwrap();
    let serr = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("encryptedfs.err")
        .unwrap();
    let mut child = Command::new("/home/gnome/dev/RustroverProjects/encryptedfs/target/debug/encryptedfs")
        .stdout(sout)
        .stderr(serr)
        .env("ENCRYPTED_FS_PASSWORD", "pass-42")
        .arg("--mount-point")
        .arg("/home/gnome/encryptedfs")
        .arg("--data-dir")
        .arg("/home/gnome/encryptedfs_data")
        .arg("--umount-on-start")

        .spawn()
        .expect("Failed to start process");

    thread::spawn(|| {
        thread::sleep(std::time::Duration::from_secs(10));
    }).join().expect("Thread panicked");
    child.kill().await.unwrap();
    println!("killed");

    thread::spawn(|| {
        thread::sleep(std::time::Duration::from_secs(60));
    }).join().expect("Thread panicked");
}