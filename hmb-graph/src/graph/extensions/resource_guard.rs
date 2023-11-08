use std::collections::HashMap;
use std::marker::PhantomData;

use async_graphql::{Context, Guard, OutputType};
use async_graphql::async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceSnippet {
    AllField {
        typename: String,
    },
    IncludeFields {
        typename: String,
        fields: Vec<String>,
    },
}

impl ResourceSnippet {
    pub fn all_fields<S: Into<String>>(s: S) -> Self {
        ResourceSnippet::AllField { typename: s.into() }
    }
    pub fn include_fields<S, II, I>(s: S, includes: I) -> Self
    where
        S: Into<String>,
        II: Into<String>,
        I: IntoIterator<Item=II>,
    {
        ResourceSnippet::IncludeFields {
            typename: s.into(),
            fields: includes.into_iter().map(|x| x.into()).collect(),
        }
    }
    pub fn typename(&self) -> &String {
        match self {
            ResourceSnippet::AllField { typename, } => typename,
            ResourceSnippet::IncludeFields { typename, fields: _ } => typename,
        }
    }
    pub fn check(&self, name: &str, field: &str) -> bool {
        match self {
            ResourceSnippet::AllField { typename } => typename == name,
            ResourceSnippet::IncludeFields { typename, fields } => {
                if typename != name {
                    return false;
                }
                return fields.iter().any(|x| x == field);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalResourceData(pub HashMap<String, ResourceSnippet>);

impl PersonalResourceData {
    pub fn new(data: HashMap<String, ResourceSnippet>) -> Self {
        Self(data)
    }
    pub fn new_iter<I: IntoIterator<Item=ResourceSnippet>>(data: I) -> Self {
        Self(data.into_iter().map(|x: ResourceSnippet| (x.typename().clone(), x)).collect::<HashMap<_, _>>())
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGuard<O: OutputType>(PhantomData<O>);

impl<O: OutputType> ResourceGuard<O> { pub fn new() -> Self { Self(PhantomData) } }


#[async_trait]
impl<O: OutputType> Guard for ResourceGuard<O> {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        let data = ctx.data_opt::<PersonalResourceData>()
                      .ok_or_else(|| async_graphql::Error {
                          message: "Don't have any guard data, access deny".to_string(),
                          source: None,
                          extensions: None,
                      })?;
        let typename = O::type_name();
        //
        match data.0.get(typename.as_ref()) {
            None => Err(async_graphql::Error {
                message: format!("Don't have guard data for :{}", typename),
                source: None,
                extensions: None,
            }),
            Some(data) => {
                let fieldname = ctx.field().name();
                if data.check(typename.as_ref(), fieldname) {
                    Ok(())
                } else {
                    Err(async_graphql::Error {
                        message: format!("you don't have access {}.{}", typename, fieldname),
                        source: None,
                        extensions: None,
                    })
                }
            }
        }
    }
}