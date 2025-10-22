use tokio_postgres::{NoTls, Error};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct ScheduledEvent {
    pub id: String,
    pub body: String,
    pub schedule_time: DateTime<Utc>,
    pub destination: String,
}

pub struct EventStore {
  client: tokio_postgres::Client,
}

impl EventStore {
    pub async fn connect() -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        Ok(EventStore { client })
    }

    pub async fn schedule_event(&self, event: String, schedule_time: DateTime<Utc>, destination: String) -> Result<String, tokio_postgres::Error> {
        let uuid = Uuid::new_v4().to_string();
        let schedule_time = schedule_time.timestamp();
        let query = "INSERT INTO events (id, body, schedule_time, destination) VALUES ($1, $2, $3, $4)";
        self.client.execute(query, &[&uuid, &event, &schedule_time, &destination]).await?;
        Ok(uuid)
    }
    
    pub async fn get_events(&self, time: String) -> Result<Vec<ScheduledEvent>, ()> {
        let query = "SELECT id, body, schedule_time, destination FROM events WHERE schedule_time = $1";
        let rows = self.client.query(query, &[&time]).await.map_err(|_| ())?;

        Ok(rows.iter().map(|row| ScheduledEvent {
            id: row.get(0),
            body: row.get(1),
            schedule_time: DateTime::<Utc>::from_timestamp_secs(row.get(2)).unwrap(),
            destination: row.get(3),
        }).collect())

    }
}