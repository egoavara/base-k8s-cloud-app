use temporal_sdk_core::protos::coresdk::activity_result::ActivityResolution;

macro_rules! wait_json_success {
    ($t:ty : $x:expr) => {
        match $x.await.status {
            | Some(Status::Completed(Success { result: Some(data) })) => match serde_json::from_slice::<$t>(data.data.as_slice()) {
                | Ok(v) => v,
                | Err(e) => return Err(anyhow!("failed to deserialize : {:?}", e)),
            },
            | Some(Status::Completed(Success { result: None })) => {
                return Err(anyhow!("missing result"));
            }
            | Some(Status::Failed(_)) => {
                return Err(anyhow!("failed"));
            }
            | Some(Status::Cancelled(_)) => {
                return Err(anyhow!("cancelled"));
            }
            | Some(Status::Backoff(_)) => {
                return Err(anyhow!("backoff"));
            }
            | None => {
                return Err(anyhow!("missing status"));
            }
        }
    };
}
pub(crate) use wait_json_success;

pub trait WfContextExt {
    fn generate_uuid_v4(&self) -> ActivityResolution;
}
