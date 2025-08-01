use airdrop::{tpl::message::MsgTemplate, utils::utils::verify_personal_sign};
use chrono::{Local};
use ethers::{
    signers::{LocalWallet, Signer}, 
    utils::hex
};


#[tokio::test]
async fn test_verify_personal_sign() {
    // 1. 随机生成一个钱包
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    let address = wallet.address();

    // 2. 构造 MetaMask 风格的签名消息
    let now  = Local::now();
    let message = MsgTemplate::Welcome(now.timestamp() as u64).format();

    let raw_message = message;
    let eth_message = format!("\x19Ethereum Signed Message:\n{}{}", raw_message.len(), raw_message);

    // 3. 签名消息（异步）
    let signature = wallet.sign_message(eth_message.clone()).await.unwrap();

    let signature_hex = format!("0x{}", hex::encode(signature.to_vec()));
    

    println!("{}",serde_json::json!({
        "address":&format!("{:?}", address),
        "sign":signature_hex.clone(),
        "timestamp":now.timestamp(),
    }));

    // 4. 验签：恢复地址
    let is_valid = verify_personal_sign(&raw_message, &signature_hex, &format!("{:?}", address)).unwrap();
    assert!(is_valid);
}