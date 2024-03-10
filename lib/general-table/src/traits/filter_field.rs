use sea_query::{IntoColumnRef, SimpleExpr};

pub(crate) fn escape_like_pg(text: impl AsRef<str>) -> String {
    text.as_ref().replace("%", "\\%").replace("_", "\\_")
}

pub trait FilterField {
    /// Apply the filter to the query builder
    /// builder : where and builder
    /// ```rust
    /// let mut builder = sqlx::QueryBuilder::new("...");
    /// let your_filter = CustomFilter::new();
    /// builder.push(" where ");
    /// let mut where_and = builder.push_sep(" and ");
    /// your_filter.apply_filter(&mut where_and, "column_name");
    /// ```
    // fn apply_filter<'a, 'qb, 'args: 'qb, Sep: Display>(&'args self, builder: &'a mut Separated<'qb, 'args, Postgres, Sep>, column: impl AsRef<str>);
    fn to_expr(&self, column: impl IntoColumnRef + Clone) -> Option<SimpleExpr>;
}
