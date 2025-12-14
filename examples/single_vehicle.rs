use bb_vpki::vehicle::OnBoardUnit;

#[tokio::main]
async fn main() {
    println!("Single Vehicle Demo\n");

    let obu = OnBoardUnit::new("VEH-DEMO-001".to_string()).await;
    println!("✓ OBU initialized for vehicle: {}", obu.vehicle_id);

    let message = b"Hello from vehicle!";
    if let Ok(signature) = obu.sign_message(message).await {
        println!("✓ Message signed: {} bytes", signature.len());

        let verified = obu.verify_message(message, &signature, &obu.public_key);
        println!("✓ Signature verified: {}", verified);
    }
}
