use std::sync::Arc;

use async_graphql::{Positioned, ServerResult, Value, Variables};
use async_graphql::extensions::{ExtensionContext, NextParseQuery, NextResolve, ResolveInfo};
use async_graphql::parser::types::{Directive, ExecutableDocument, FragmentDefinition, OperationDefinition, Selection};
use tracing::{info, warn};

pub struct GraphGuard;

pub struct GraphGuardExtension;

impl async_graphql::extensions::ExtensionFactory for GraphGuard {
    fn create(&self) -> Arc<dyn async_graphql::extensions::Extension> {
        Arc::new(GraphGuardExtension)
    }
}

#[async_trait::async_trait]
impl async_graphql::extensions::Extension for GraphGuardExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let parsed = next.run(ctx, query, variables).await?;
        // println!("query_type : {:?}", ctx.schema_env.registry.query_type.keys());
        println!("types : {:?}", ctx.schema_env.registry.types.keys());
        println!("query_type : {:?}", ctx.schema_env.registry.query_type);
        println!("mutation_type : {:?}", ctx.schema_env.registry.mutation_type);
        println!("subscription_type : {:?}", ctx.schema_env.registry.subscription_type);
        // info!("ctx: {:#?}", ctx);
        info!("parsed: {:#?}", parsed);
        let directives = RebacDirectivesRSearcher::rsearch_rebac(&parsed)
            .end()?;
        info!("directives: {:?}", directives);
        Ok(parsed)
    }
    /// Called at resolve field.
    async fn resolve(
        &self,
        ctx: &ExtensionContext<'_>,
        info: ResolveInfo<'_>,
        next: NextResolve<'_>,
    ) -> ServerResult<Option<Value>> {

        info!("resolve");
        // info!("field: {:#?}", ctx.schema_env.registry.types.keys());
        // // info!("field: {:#?}", ctx.schema_env.registry.types.);
        // info!("field: {:#?}", info.parent_type);
        // info!("field: {:#?}", info.return_type);
        // info!("field: {:#?}", info.field);
        next.run(ctx, info).await
    }
}


struct RebacDirectivesRSearcher<'a> {
    directives: Vec<&'a Positioned<Directive>>,
}

#[derive(Debug)]
struct RebacDirective {
    rel: String,
    otype: String,
    oid: String,
    result: bool,
}

impl<'a> RebacDirectivesRSearcher<'a> {
    fn rsearch_rebac(doc: &'a ExecutableDocument) -> Self {
        let mut _self = Self {
            directives: Vec::new(),
        };
        doc.operations
            .iter()
            .for_each(|(_, def)| {
                _self.rsearch_rabac_opdef(&def.node)
            });
        doc.fragments
            .iter()
            .for_each(|(_, def)| {
                _self.rsearch_rabac_fragdef(&def.node)
            });
        _self
    }
    fn rsearch_rabac_opdef(&mut self, def: &'a OperationDefinition) {
        def.directives
            .iter()
            .filter(|&d| d.node.name.node == "rebac")
            .for_each(|d| {
                self.directives.push(d)
            });
        def.selection_set.node.items.iter()
            .for_each(|s| {
                self.rsearch_rabac_selection(&s.node)
            })
    }
    fn rsearch_rabac_fragdef(&mut self, def: &'a FragmentDefinition) {
        def.directives
            .iter()
            .filter(|&d| d.node.name.node == "rebac")
            .for_each(|d| {
                self.directives.push(d)
            });
        def.selection_set.node.items.iter()
            .for_each(|s| {
                self.rsearch_rabac_selection(&s.node)
            })
    }
    fn rsearch_rabac_selection(&mut self, def: &'a Selection) {
        def.directives()
            .iter()
            .filter(|&d| d.node.name.node == "rebac")
            .for_each(|d| {
                self.directives.push(d)
            });
        match def {
            Selection::Field(field) => {
                field.node.selection_set.node.items.iter()
                    .for_each(|s| {
                        self.rsearch_rabac_selection(&s.node)
                    })
            }
            Selection::InlineFragment(ifrag) => {
                ifrag.node.selection_set.node.items.iter()
                    .for_each(|s| {
                        self.rsearch_rabac_selection(&s.node)
                    })
            }
            Selection::FragmentSpread(_) => {}
        }
    }
    fn end(self) -> Result<Vec<RebacDirective>, async_graphql::ServerError> {
        self.directives
            .into_iter()
            .map(|x| {
                let mut rel: Result<Option<String>, async_graphql::ServerError> = Ok(None);
                let mut otype: Result<Option<String>, async_graphql::ServerError> = Ok(None);
                let mut oid: Result<Option<String>, async_graphql::ServerError> = Ok(None);
                let mut result: Result<Option<bool>, async_graphql::ServerError> = Ok(None);

                for (name, value) in &x.node.arguments {
                    match name.node.as_str() {
                        "rel" => {
                            rel = rel.and_then(|x| match (x, &value.node) {
                                (Some(_), _) => Err(async_graphql::ServerError::new("Duplicate argument rel", Some(name.pos))),
                                (None, async_graphql_value::Value::String(str)) => Ok(Some(str.to_string())),
                                (None, v) => Err(async_graphql::ServerError::new("Argument rel must be a string", Some(value.pos))),
                            });
                        }
                        "otype" => {
                            otype = otype.and_then(|x| match (x, &value.node) {
                                (Some(_), _) => Err(async_graphql::ServerError::new("Duplicate argument otype", Some(name.pos))),
                                (None, async_graphql_value::Value::String(str)) => Ok(Some(str.to_string())),
                                (None, v) => Err(async_graphql::ServerError::new("Argument otype must be a string", Some(value.pos))),
                            });
                        }
                        "oid" => {
                            oid = oid.and_then(|x| match (x, &value.node) {
                                (Some(_), _) => Err(async_graphql::ServerError::new("Duplicate argument oid", Some(name.pos))),
                                (None, async_graphql_value::Value::String(str)) => Ok(Some(str.to_string())),
                                (None, v) => Err(async_graphql::ServerError::new("Argument oid must be a string", Some(value.pos))),
                            });
                        }
                        "result" => {
                            result = result.and_then(|x| match (x, &value.node) {
                                (Some(_), _) => Err(async_graphql::ServerError::new("Duplicate argument result", Some(name.pos))),
                                (None, async_graphql_value::Value::Boolean(b)) => Ok(Some(*b)),
                                (None, v) => Err(async_graphql::ServerError::new("Argument result must be a boolean", Some(value.pos))),
                            });
                        }
                        _ => {
                            warn!("Unknown argument {:?} with value {:?}", name.node, value.node)
                        }
                    }
                }

                match (rel?, otype?, oid?, result?) {
                    (Some(rel), Some(otype), Some(oid), Some(result)) => {
                        Ok(RebacDirective {
                            rel,
                            otype,
                            oid,
                            result,
                        })
                    }
                    _ => {
                        Err(async_graphql::ServerError::new("Missing required argument one of rel, otype, oid, result", Some(x.pos)))
                    }
                }
            })
            .collect()
    }
}