use ethers::core::utils::hash_message;
use ethers::types::{Address, Signature};


pub fn verify_personal_sign(message: &str, signature: &str, claimed_address: &str) -> Result<bool, Box<dyn std::error::Error>> {

    // 1. 将签名从 hex 解析为 Signature
    let signature: Signature = signature.parse()?;

    // 2. 对消息进行 personal_sign 格式的哈希（即 "\x19Ethereum Signed Message:\n" + len + message）
    let eth_message = format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message);
    let message_hash = hash_message(eth_message);

    // 3. 使用签名恢复地址
    let recovered_address = signature.recover(message_hash)?;

    // 4. 比较恢复出来的地址和声明的地址
    let expected_address: Address = claimed_address.parse()?;

    Ok(recovered_address == expected_address)
}


