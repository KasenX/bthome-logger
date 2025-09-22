use sqlx::SqlitePool;

pub async fn insert_sample(
    pool: &SqlitePool,
    address: &str,
    packet_counter: Option<u8>,
    temperature: Option<f32>,
    humidity: Option<f32>,
    battery: Option<u8>,
) -> sqlx::Result<i64> {
    sqlx::query_scalar!(
        r#"
            INSERT INTO
                samples (
                    address,
                    packet_counter,
                    temperature,
                    humidity,
                    battery
                )
            VALUES
                (?1, ?2, ?3, ?4, ?5)
            RETURNING id
        "#,
        address,
        packet_counter,
        temperature,
        humidity,
        battery,
    )
    .fetch_one(pool)
    .await
}
