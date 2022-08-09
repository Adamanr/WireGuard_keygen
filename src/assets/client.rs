pub mod api{
    use std::{fs, str};
    use tokio::process::Command;

    pub async fn generate_private_key(ip:&str) -> String{
        let output = Command::new("bash")
            .args(&["-c",format!("wg genkey > privatekey_{}  && cat privatekey_{}",ip,ip).as_str()])
            .output().await
            .expect("Failed to execute command");
        String::from_utf8_lossy(output.stdout.as_slice()).to_string()
    }

    pub async fn generate_public_key(ip:&str) -> String{
        let output = Command::new("bash")
            .args(&["-c",format!("wg pubkey < privatekey_{} > publickey_{}  && cat publickey_{}",ip,ip,ip).as_str()])
            .output().await
            .expect("Failed to execute command");
        String::from_utf8_lossy(output.stdout.as_slice()).to_string()
    }

    pub async fn add_client(pb_key: &String, ip:&str){
        Command::new("bash")
            .arg("-c")
            .arg(format!("wg set wg0 peer {} allowed-ips {}/32",pb_key.trim(),ip))
            .output().await.expect("Error add client");
        Command::new("bash")
            .arg("-c")
            .arg("wg-quick save /etc/wireguard/wg0.conf")
            .output().await.expect("Error save!");
    }

    pub async fn delete_client(pb_key:String){
        Command::new("bash")
            .arg("-c")
            .arg(format!("wg set wg0 peer {} remove",pb_key))
            .output().await.expect("Error wg set!");
        Command::new("bash")
            .arg("-c")
            .arg("wg-quick save /etc/wireguard/wg0.conf")
            .output().await.expect("Error save");
    }

    pub async fn read_key(ip:&str)->(String,String) {
        let private_key  = generate_private_key(ip).await;
        let public_key = generate_public_key(ip).await;
        add_client(&public_key, ip).await;
        fs::remove_file(format!("publickey_{}",ip).as_str()).expect("Error remove file");
        fs::remove_file(format!("privatekey_{}",ip).as_str()).expect("Error remove file");
        println!("Ключи отданы");
        (private_key.trim().parse().unwrap(), public_key.trim().parse().unwrap())
    }
}
