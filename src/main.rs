use communications::build_run;

#[tokio::main]
async fn main() {
    build_run().await;
}

#[tokio::test]
async fn quick_debug() -> anyhow::Result<()> {
    tokio::spawn(build_run());
    let hc = httpc_test::new_client("http://localhost:3030")?;

    hc.do_post("/mailer/send_mail", serde_json::json!({
        "to": "ratneshjain40@gmail.com",
        "subject": "test",
        "body": "test"
    })).await?.print().await?;

    Ok(())
}