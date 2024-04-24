use crate::utils::Empty;
use darling::util::Override;
use darling::{ast, FromDeriveInput, FromField, FromMeta};
use derivative::Derivative;
use syn::spanned::Spanned;
use syn::Path;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(table), supports(struct_named))]
pub struct Table {
    pub(crate) ident: syn::Ident,

    pub(crate) schema: Option<String>,
    pub(crate) table: Option<String>,
    pub(crate) history: Option<TableHistory>,

    #[darling(default)]
    pub(crate) naming: TableNaming,

    pub(crate) data: ast::Data<(), Column>,
}

#[derive(Debug, FromMeta)]
pub enum Case {
    Pascal,
    Camel,
    Snake,
}

#[derive(Debug, FromMeta)]
pub struct TableHistory {
    pub(crate) schema: Option<String>,
    pub(crate) table: Option<String>,
    #[darling(default)]
    pub(crate) mode: TableHistoryMode,
}

#[derive(Debug, FromMeta, Derivative)]
#[derivative(Default)]
pub struct TableNaming {
    #[derivative(Default(value = r#"String::from("${table}Field")"#))]
    pub(crate) field: String,

    #[derivative(Default(value = r#"String::from("${table}Filter")"#))]
    pub(crate) filter: String,

    #[derivative(Default(value = r#"String::from("${table}Cursor")"#))]
    pub(crate) cursor: String,

    #[derivative(Default(value = r#"String::from("${table}Sorter")"#))]
    pub(crate) sorter: String,

    #[derivative(Default(value = r#"String::from("${table}SorterElem")"#))]
    pub(crate) sorter_elem: String,

    #[derivative(Default(value = r#"String::from("${table}Create")"#))]
    pub(crate) create: String,

    #[derivative(Default(value = r#"String::from("${table}Update")"#))]
    pub(crate) update: String,

    #[derivative(Default(value = r#"String::from("${table}Filter${column}Locality")"#))]
    pub(crate) locality_filter: String,

    #[derivative(Default(value = r#"String::from("${table}Sorter{column}Locality")"#))]
    pub(crate) locality_sorter: String,

    #[derivative(Default(value = r#"Case::Snake"#))]
    pub(crate) field_case: Case,

    #[derivative(Default(value = r#"Case::Pascal"#))]
    pub(crate) enum_case: Case,

    #[derivative(Default(value = r#"Case::Snake"#))]
    pub(crate) db_case: Case,
}

#[derive(Debug, FromMeta, Derivative)]
#[derivative(Default)]
pub enum TableHistoryMode {
    #[derivative(Default)]
    Auto(Override<Empty>),
    Snapshot(Override<TableHistoryModeSnapshot>),
    Audit(Override<TableHistoryModeAudit>),
    Mix(Override<TableHistoryModeMix>),
}

#[derive(Debug, FromMeta)]
pub struct TableHistoryModeSnapshot {}

#[derive(Debug, FromMeta)]
pub struct TableHistoryModeAudit {}

#[derive(Debug, FromMeta)]
pub struct TableHistoryModeMix {}

#[derive(Debug, FromField)]
#[darling(attributes(column), and_then = Column::validation)]
pub struct Column {
    pub(crate) ident: Option<syn::Ident>,
    pub(crate) ty: syn::Type,
    #[darling(default)]
    pub(crate) id: bool,
    pub(crate) filter: Option<Override<ColumnFilter>>,
    pub(crate) sorter: Option<Override<ColumnSorter>>,
    pub(crate) create: Option<Override<ColumnCreate>>,
    pub(crate) update: Option<Override<ColumnUpdate>>,
    pub(crate) delete: Option<Override<ColumnDelete>>,
}

impl Column {
    fn span(&self) -> proc_macro2::Span {
        self.ident
            .as_ref()
            .map_or_else(|| self.ty.span(), |ident| ident.span())
    }
    pub fn validation(mut self) -> darling::Result<Self> {
        match (self.id, &self.filter) {
            (false, _)
            | (
                true,
                // must one of no filter, inherit filter, default filter
                Some(Override::Inherit) | Some(Override::Explicit(ColumnFilter::Default)),
            ) => {}
            (true, None) => {
                // set default filter
                self.filter = Some(Override::Inherit);
            }
            (true, _) => {
                return Err(darling::Error::custom(
                    "id column must use inherit filter, do not use with or by filter",
                )
                .with_span(&self.ident));
            }
            (_, _) => {}
        }

        match &self.sorter {
            Some(Override::Explicit(ColumnSorter::With(sorter_with))) => {
                sorter_with.validation(&self)?;
            }
            _ => {}
        }
        Ok(self)
    }
}

#[derive(Debug, FromMeta)]
pub enum ColumnFilter {
    Default,
    With(ColumnFilterWith),
    By(Path),
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ColumnFilterWith {
    #[darling(default)]
    pub(crate) eq: bool,
    #[darling(default)]
    pub(crate) ne: bool,
    #[darling(default)]
    pub(crate) gt: bool,
    #[darling(default)]
    pub(crate) lt: bool,
    #[darling(default)]
    pub(crate) gte: bool,
    #[darling(default)]
    pub(crate) lte: bool,
    #[darling(default)]
    pub(crate) r#in: bool,
    #[darling(default)]
    pub(crate) not_in: bool,
    #[darling(default)]
    pub(crate) like: bool,
    #[darling(default)]
    pub(crate) nlike: bool,
    #[darling(default)]
    pub(crate) null: bool,
    #[darling(default)]
    pub(crate) not_null: bool,
    #[darling(default)]
    pub(crate) between: bool,
    #[darling(default)]
    pub(crate) nbetween: bool,
    #[darling(default)]
    pub(crate) prefix: bool,
    #[darling(default)]
    pub(crate) nprefix: bool,
    #[darling(default)]
    pub(crate) suffix: bool,
    #[darling(default)]
    pub(crate) nsuffix: bool,
    #[darling(default)]
    pub(crate) contains: bool,
    #[darling(default)]
    pub(crate) ncontains: bool,
    #[darling(default)]
    pub(crate) regex: bool,
}

#[derive(Debug, FromMeta)]
pub enum ColumnSorter {
    Default,
    With(ColumnSorterWith),
    By(Path),
}

#[derive(Debug, Default, Clone, FromMeta)]
pub struct ColumnSorterWith {
    #[darling(default)]
    pub(crate) asc: bool,
    #[darling(default)]
    pub(crate) desc: bool,
    #[darling(default)]
    pub(crate) values: bool,
}

impl ColumnSorterWith {
    pub fn validation(&self, parent: &Column) -> darling::Result<()> {
        if !self.asc && !self.desc {
            return Err(darling::Error::custom("at lease set asc or desc required")
                .with_span(&parent.span()));
        }
        Ok(())
    }

    pub fn is_simple_order(&self) -> bool {
        !self.values
    }
}

#[derive(Debug, FromMeta)]
pub struct ColumnCreate {
    /// init only when the column is null
    #[darling(default)]
    pub(crate) lazy_init: bool,
}

#[derive(Debug, FromMeta)]
pub struct ColumnUpdate {
    /// revert the column previous value
    #[darling(default)]
    pub(crate) revert: bool,
}

#[derive(Debug, FromMeta)]
pub struct ColumnDelete {}

#[cfg(test)]
mod test {
    use darling::FromDeriveInput;
    use quote::quote;

    use crate::derive_table::Table;

    #[test]
    fn table_full() {
        let table = quote! {
            #[derive(Table)]
            #[table(schema = "public", table = "test", history(schema = "public", table = "test_audit", mode(mix)))]
            pub struct Test {
                #[column(id)]
                pub id: Uuid,
                #[column(update, filter)]
                pub name_inherit: String,
                #[column(filter(with(eq, ne)))]
                pub name_with: String,
                #[column(filter(by = String))]
                pub name_use: String,

                #[column(create, update)]
                pub create_normal: String,
                #[column(create(lazy_init))]
                pub create_lazy: String,
                #[column(update(revert))]
                pub update_revert: String,
            }
        };
        let source = syn::parse2(table).unwrap();
        let actual = Table::from_derive_input(&source).unwrap();
        let actual = format!("{:#?}", actual);

        let expect = r#"Table {
    ident: Ident(
        Test,
    ),
    schema: None,
    table: None,
    history: Some(
        Audit(
            TableHistoryAudit {
                schema: Some(
                    "public",
                ),
                table: Some(
                    "test_audit",
                ),
            },
        ),
    ),
    data: Struct(
        Fields {
            style: Struct,
            fields: [
                Column {
                    ident: Some(
                        Ident(
                            id,
                        ),
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        Uuid,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                    id: true,
                    filter: None,
                    sorter: None,
                },
            ],
            span: Some(
                Span,
            ),
            __nonexhaustive: (),
        },
    ),
}"#;
        pretty_assertions::assert_eq!(actual, expect);
    }

    #[test]
    fn table_basic() {
        let table = quote! {
            #[derive(Table)]
            #[table(schema = "public", table = "test")]
            pub struct Test {
                #[column(id)]
                pub id: Uuid,
            }
        };
        let source = syn::parse2(table).unwrap();
        let actual = Table::from_derive_input(&source).unwrap();
        let actual = format!("{:#?}", actual);
        let expect = r#"Table {
    ident: Ident(
        Test,
    ),
    schema: Some(
        "public",
    ),
    table: Some(
        "test",
    ),
    history: None,
    data: Struct(
        Fields {
            style: Struct,
            fields: [
                Column {
                    ident: Some(
                        Ident(
                            id,
                        ),
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        Uuid,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                    id: true,
                    filter: None,
                    sorter: None,
                },
            ],
            span: Some(
                Span,
            ),
            __nonexhaustive: (),
        },
    ),
}"#;
        pretty_assertions::assert_eq!(actual, expect);
    }

    #[test]
    fn table_history_implicit() {
        let table = quote! {
            #[derive(Table)]
            #[table(history(schema = "public", table = "test_audit"))]
            pub struct Test {
                #[column(id)]
                pub id: Uuid,
            }
        };
        let source = syn::parse2(table).unwrap();
        let actual = Table::from_derive_input(&source).unwrap();
        let actual = format!("{:#?}", actual);
        println!("{}", actual);
        let expect = r#"Table {
    ident: Ident(
        Test,
    ),
    schema: None,
    table: None,
    history: Some(
        TableHistory {
            schema: Some(
                "public",
            ),
            table: Some(
                "test_audit",
            ),
            mode: Auto(
                Inherit,
            ),
        },
    ),
    data: Struct(
        Fields {
            style: Struct,
            fields: [
                Column {
                    ident: Some(
                        Ident(
                            id,
                        ),
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        Uuid,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                    id: true,
                    filter: None,
                    sorter: None,
                },
            ],
            span: Some(
                Span,
            ),
            __nonexhaustive: (),
        },
    ),
}"#;
        pretty_assertions::assert_eq!(actual, expect);
    }

    #[test]
    fn table_history_auto() {
        let table = quote! {
            #[derive(Table)]
            #[table(history(schema = "public", table = "test_audit", mode(auto)))]
            pub struct Test {
                #[column(id)]
                pub id: Uuid,
            }
        };
        let source = syn::parse2(table).unwrap();
        let actual = Table::from_derive_input(&source).unwrap();
        let actual = format!("{:#?}", actual);
        let expect = r#"Table {
    ident: Ident(
        Test,
    ),
    schema: None,
    table: None,
    history: Some(
        TableHistory {
            schema: Some(
                "public",
            ),
            table: Some(
                "test_audit",
            ),
            mode: Auto(
                Inherit,
            ),
        },
    ),
    data: Struct(
        Fields {
            style: Struct,
            fields: [
                Column {
                    ident: Some(
                        Ident(
                            id,
                        ),
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        Uuid,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                    id: true,
                    filter: None,
                    sorter: None,
                },
            ],
            span: Some(
                Span,
            ),
            __nonexhaustive: (),
        },
    ),
}"#;
        pretty_assertions::assert_eq!(actual, expect);
    }

    #[test]
    fn table_history_snapshot_implicit() {
        let table = quote! {
            #[derive(Table)]
            #[table(history(schema = "public", table = "test_snapshot", mode(snapshot)))]
            pub struct Test {
                #[column(id)]
                pub id: Uuid,
            }
        };
        let source = syn::parse2(table).unwrap();
        let actual = Table::from_derive_input(&source).unwrap();
        let actual = format!("{:#?}", actual);
        let expect = r#"Table {
    ident: Ident(
        Test,
    ),
    schema: None,
    table: None,
    history: Some(
        TableHistory {
            schema: Some(
                "public",
            ),
            table: Some(
                "test_snapshot",
            ),
            mode: Snapshot(
                Inherit,
            ),
        },
    ),
    data: Struct(
        Fields {
            style: Struct,
            fields: [
                Column {
                    ident: Some(
                        Ident(
                            id,
                        ),
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        Uuid,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                    id: true,
                    filter: None,
                    sorter: None,
                },
            ],
            span: Some(
                Span,
            ),
            __nonexhaustive: (),
        },
    ),
}"#;
        pretty_assertions::assert_eq!(actual, expect);
    }

    #[test]
    fn table_filter() {
        let table = quote! {
            #[derive(Table)]
            pub struct Test {
                #[column(id)]
                pub id: Uuid,
                #[column(filter)]
                pub name_inherit: String,
                #[column(filter(with(eq, ne)))]
                pub name_with: String,
                #[column(filter(by = String))]
                pub name_use: String,
            }
        };
        let source = syn::parse2(table).unwrap();
        let actual = Table::from_derive_input(&source).unwrap();
        let actual = format!("{:#?}", actual);

        let expect = r#"Table {
    ident: Ident(
        Test,
    ),
    schema: None,
    table: None,
    history: Some(
        Audit(
            TableHistoryAudit {
                schema: Some(
                    "public",
                ),
                table: Some(
                    "test_audit",
                ),
            },
        ),
    ),
    data: Struct(
        Fields {
            style: Struct,
            fields: [
                Column {
                    ident: Some(
                        Ident(
                            id,
                        ),
                    ),
                    ty: Type::Path {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        Uuid,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                    },
                    id: true,
                    filter: None,
                    sorter: None,
                },
            ],
            span: Some(
                Span,
            ),
            __nonexhaustive: (),
        },
    ),
}"#;
        pretty_assertions::assert_eq!(actual, expect);
    }
}
