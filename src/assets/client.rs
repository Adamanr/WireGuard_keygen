pub mod api{
    use std::{process::Command, str};
    use std::fs::File;
    use std::io::Read;
    use std::thread::sleep;
    use std::time::Duration;
    use std::path::Path;

    pub async fn generate_private_key(){
        Command::new("bash")
            .arg("-c")
            .arg("wg genkey > privatekey")
            .spawn().ok().expect("Error");
    }

    pub async fn generate_public_key(){
        Command::new("bash")
            .arg("-c")
            .arg("wg pubkey < privatekey > publickey")
            .spawn().ok().expect("Error");
    }

    pub fn add_client(pb_key: &str, ip:&str){
        Command::new("bash")
            .arg("-c")
            .arg(format!("wg set wg0 peer {} {}",pb_key,ip))
            .spawn().ok().expect("Error");
    }

    pub fn save_client(){
        Command::new("bash")
            .arg("-c")
            .arg("wg-quick save /etc/wireguard/wg0.conf")
            .spawn().ok().expect("Error");
    }

    #[allow(unused)]
    pub fn delete_client(pb_key:&str){
        Command::new("bash")
            .arg("-c")
            .arg("wg set wg0 peer base64_client_publickey remove")
            .spawn().ok().expect("Error");

        Command::new("bash")
            .arg("-c")
            .arg("wg-quick save /etc/wireguard/wg0.conf")
            .spawn().ok().expect("Error");

    }

    pub async fn read_key(ip:&str)->(String,String) {
        generate_public_key().await;
        generate_private_key().await;
        let mut private_key = String::new();
        let mut public_key = String::new();

        while !Path::new("privatekey").exists() && !Path::new("publickey").exists()  {
            sleep(Duration::from_millis(800));
        }
        sleep(Duration::from_millis(500));
        let mut file = File::open("privatekey").expect("Error find privatekey file");
        file.read_to_string(&mut private_key).unwrap();

        let mut file = File::open("publickey").expect("Error find publickey file");
        file.read_to_string(&mut public_key).unwrap();

        add_client(public_key.as_str(), ip);
        save_client();
        (private_key.trim().parse().unwrap(), public_key.trim().parse().unwrap())
    }
}