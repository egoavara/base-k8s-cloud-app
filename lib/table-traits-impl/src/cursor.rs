use serde::de::DeserializeOwned;

#[derive(Debug, Default, Clone)]
pub struct Cursor<T> {
    after: Option<T>,
    first: Option<u32>,
    before: Option<T>,
    last: Option<u32>,
}

impl<DE: DeserializeOwned> Cursor<DE> {
    pub fn new(
        after: Option<String>,
        first: Option<u32>,
        before: Option<String>,
        last: Option<u32>,
    ) -> Result<Cursor<DE>, serde_qs::Error> {
        let after = after.map_or(Ok(None), |x| serde_qs::from_str::<DE>(x.as_str()).map(Some))?;
        let before = before.map_or(Ok(None), |x| serde_qs::from_str::<DE>(x.as_str()).map(Some))?;
        Ok(Cursor {
            after,
            first,
            before,
            last,
        })
    }
}
