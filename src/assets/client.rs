pub mod api{
    use std::{process::Command, str};
    use std::fs::File;
    use std::io::Read;

    pub async fn generate_private_key(ip:&str) -> String{
        let file_name = format!("privatekey_{}",ip);
        let gen_private = format!("wg genkey > {}",file_name);
        Command::new("bash")
            .arg("-c")
            .arg(gen_private.as_str())
            .spawn().ok().expect("Error generate private_key");
        file_name
    }

    pub async fn generate_public_key(ip:&str,private_key:&str){
        let public_key = format!("publickey_{}",ip);
        let gen_public = format!("wg pubkey < {} > {}",private_key,public_key);
        Command::new("bash")
            .arg("-c")
            .arg(gen_public.as_str())
            .spawn().ok().expect("Error generate public key");
    }

    pub fn add_client(pb_key: &str, ip:&str){
        Command::new("bash")
            .arg("-c")
            .arg(format!("wg set wg0 peer {} {}",pb_key,ip))
            .spawn().ok().expect("Error add client");
    }

    pub fn save_client(){
        Command::new("bash")
            .arg("-c")
            .arg("wg-quick save /etc/wireguard/wg0.conf")
            .spawn().ok().expect("Error save client");
    }

    pub fn delete_client(pb_key:&str){
        Command::new("bash")
            .arg("-c")
            .arg("wg set wg0 peer")
            .arg(pb_key)
            .arg("remove")
            .spawn().ok().expect("Error remove client");

        Command::new("bash")
            .arg("-c")
            .arg("wg-quick save /etc/wireguard/wg0.conf")
            .spawn().ok().expect("Error save client after remove");
    }

    pub async fn read_key(ip:&str)->(String,String) {
        let priv_key = generate_private_key(ip).await;
        generate_public_key(ip,priv_key.as_str()).await;

        let mut private_key = String::new();
        let mut public_key = String::new();

        let mut file = File::open(format!("privatekey_{}",ip).as_str()).expect("Error find privatekey file");
        file.read_to_string(&mut private_key).unwrap();
        let mut file = File::open(format!("publickey_{}",ip).as_str() ).expect("Error find publickey file");
        file.read_to_string(&mut public_key).unwrap();
        add_client(public_key.as_str(), ip);
        save_client();
        (private_key.trim().parse().unwrap(), public_key.trim().parse().unwrap())
    }
}