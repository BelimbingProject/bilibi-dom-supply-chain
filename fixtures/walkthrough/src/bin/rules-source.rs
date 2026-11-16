#[tokio::main]
async fn main() -> bilibi_database::Result<()> {
    let url = std::env::var("DATABASE_URL").map_err(|_| bilibi_database::Error::Invalid)?;
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap_or_else(|| "prepare".into());
    let order_id = args
        .next()
        .map(|id| {
            id.parse::<i64>()
                .map_err(|_| bilibi_database::Error::Invalid)
        })
        .transpose()?;
    bilibi_supply_chain_walkthrough::prepare_rules(&url, &mode, order_id).await
}
