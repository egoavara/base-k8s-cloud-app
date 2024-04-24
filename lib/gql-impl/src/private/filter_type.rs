use sea_query::extension::postgres::PgBinOper;
use sea_query::{IntoColumnRef, SimpleExpr};

pub trait FilterType: Sized + Clone {
    type Target;
    type TargetContainer;
    type TargetRange;

    // equality
    fn expr_eq(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_ne(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    // comparison
    fn expr_gt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_lt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_gte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_lte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    // set
    fn expr_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_not_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    // like
    fn expr_like(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_nlike(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    // null
    fn expr_null(col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_not_null(col: impl IntoColumnRef) -> Option<SimpleExpr>;
    // between
    fn expr_between(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_nbetween(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    // string matches
    fn expr_prefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_nprefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_suffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_nsuffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_contain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    fn expr_ncontain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
    // regex
    fn expr_regex(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr>;
}

fn str_like_escape<S: Into<String>>(str: S) -> String {
    let mut str = str.into();
    str = str.replace("\\", "\\\\");
    str = str.replace("%", "\\%");
    str = str.replace("_", "\\_");
    str
}

macro_rules! impl_default_types {
    ($t:ty = $($tt:tt)+) => {
        impl FilterType for $t {
            type Target = Self;
            type TargetContainer = std::vec::Vec<Self>;
            type TargetRange = crate::Range<Self>;
            impl_default_types!(= $($tt)+);
        }
    };
    ($t:ty $(, $et:ty)+ = $($tt:tt)+) => {
        impl FilterType for $t {
            type Target = Self;
            type TargetContainer = std::vec::Vec<Self>;
            type TargetRange = crate::Range<Self>;
            impl_default_types!(= $($tt)+);
        }
        impl_default_types!($($et),+ = $($tt)+);
    };
    (= $op:tt) => {
        impl_default_types!(impl $op);
    };
    (= $op:tt ($args:expr)) => {
        impl_default_types!(impl $op $args);
    };
    (= $op:tt + $($tt:tt)+) => {
        impl_default_types!(impl $op);
        impl_default_types!(= $($tt)+);
    };
    (= $op:tt ($args:expr) + $($tt:tt)+) => {
        impl_default_types!(impl $op $args);
        impl_default_types!(= $($tt)+);
    };

    (= ! $op:tt) => {
        impl_default_types!(impl ! $op);
    };
    (= ! $op:tt ($args:expr)) => {
        impl_default_types!(impl ! $op $args);
    };
    (= ! $op:tt + $($tt:tt)+) => {
        impl_default_types!(impl ! $op);
        impl_default_types!(= $($tt)+);
    };
    (= ! $op:tt ($args:expr) + $($tt:tt)+) => {
        impl_default_types!(impl ! $op $args);
        impl_default_types!(= $($tt)+);
    };

    (impl eq) => { fn expr_eq(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).eq(::sea_query::Expr::value(value))) }};
    (impl ! eq) => { fn expr_eq(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl ne) => { fn expr_ne(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).ne(::sea_query::Expr::value(value))) }};
    (impl ! ne) => { fn expr_ne(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};

    (impl gt) => { fn expr_gt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).gt(::sea_query::Expr::value(value))) }};
    (impl ! gt) => { fn expr_gt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl lt) => { fn expr_lt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).lt(::sea_query::Expr::value(value))) }};
    (impl ! lt) => { fn expr_lt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl gte) => { fn expr_gte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).gte(::sea_query::Expr::value(value))) }};
    (impl ! gte) => { fn expr_gte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl lte) => { fn expr_lte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).lte(::sea_query::Expr::value(value))) }};
    (impl ! lte) => { fn expr_lte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};

    (impl in) => { fn expr_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).is_in(value)) }};
    (impl in $map:expr) => { fn expr_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).is_in(value.map($map))) }};
    (impl ! in_) => { fn expr_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl not_in) => { fn expr_not_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).is_not_in(value)) }};
    (impl not_in $map:expr ) => { fn expr_not_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).is_not_in(value.map($map))) }};
    (impl ! not_in) => { fn expr_not_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};

    (impl like) => { fn expr_like(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).like(value)) }};
    (impl ! like) => { fn expr_like(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl nlike) => { fn expr_nlike(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).not_like(value)) }};
    (impl ! nlike) => { fn expr_nlike(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};


    (impl null) => { fn expr_null(col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).is_null()) }};
    (impl ! null) => { fn expr_null(col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl not_null) => { fn expr_not_null(col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).is_not_null()) }};
    (impl ! not_null) => { fn expr_not_null(col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};

    (impl between) => { fn expr_between(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).between(::sea_query::Expr::value(value.min), ::sea_query::Expr::value(value.max))) }};
    (impl ! between) => { fn expr_between(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl not_between) => { fn expr_nbetween(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).not_between(::sea_query::Expr::value(value.min), ::sea_query::Expr::value(value.max))) }};
    (impl ! not_between) => { fn expr_nbetween(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};

    (impl prefix) => { fn expr_prefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).like(format!("{}%", str_like_escape(value)))) }};
    (impl ! prefix) => { fn expr_prefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl nprefix) => { fn expr_nprefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).not_like(format!("{}%", str_like_escape(value)))) }};
    (impl ! nprefix) => { fn expr_nprefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl suffix) => { fn expr_suffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).like(format!("%{}", str_like_escape(value)))) }};
    (impl ! suffix) => { fn expr_suffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl nsuffix) => { fn expr_nsuffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).not_like(format!("%{}", str_like_escape(value)))) }};
    (impl ! nsuffix) => { fn expr_nsuffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl contain) => { fn expr_contain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).like(format!("%{}%", str_like_escape(value)))) }};
    (impl ! contain) => { fn expr_contain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
    (impl ncontain) => { fn expr_ncontain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).not_like(format!("%{}%", str_like_escape(value)))) }};
    (impl ! ncontain) => { fn expr_ncontain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};

    (impl regex) => { fn expr_regex(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { Some(::sea_query::Expr::col(col).binary(PgBinOper::Regex, ::sea_query::Expr::value(value))) }};
    (impl ! regex) => { fn expr_regex(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> { None }};
}

impl_default_types!(std::string::String = eq + ne + gt + gte + lt + lte + in + not_in + like + nlike + !null + !not_null + between + not_between + prefix + nprefix + suffix + nsuffix + contain + ncontain + regex);

#[cfg(feature = "with-uuid")]
impl_default_types!(uuid::Uuid = eq + ne + !gt + !gte + !lt + !lte + in + not_in + !like + !nlike + !null + !not_null + !between + !not_between + !prefix + !nprefix + !suffix + !nsuffix + !contain + !ncontain + !regex);
//
impl_default_types!(
    u8,
    u16,
    u32,
    u64 = eq + ne + gt + gte + lt + lte + !in_ + !not_in + !like + !nlike + !null + !not_null + between + not_between + !prefix + !nprefix + !suffix + !nsuffix + !contain + !ncontain + !regex
);
impl_default_types!(
    i8,
    i16,
    i32,
    i64 = eq + ne + gt + gte + lt + lte + !in_ + !not_in + !like + !nlike + !null + !not_null + between + not_between + !prefix + !nprefix + !suffix + !nsuffix + !contain + !ncontain + !regex
);

impl<T: FilterType> FilterType for Option<T> {
    type Target = Option<T::Target>;
    type TargetContainer = Option<T::TargetContainer>;
    type TargetRange = Option<T::TargetRange>;

    fn expr_eq(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_eq(value, col),
            None => None,
        }
    }

    fn expr_ne(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_ne(value, col),
            None => None,
        }
    }

    fn expr_gt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_gt(value, col),
            None => None,
        }
    }

    fn expr_lt(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_lt(value, col),
            None => None,
        }
    }

    fn expr_gte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_gte(value, col),
            None => None,
        }
    }

    fn expr_lte(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_lte(value, col),
            None => None,
        }
    }

    fn expr_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_in(value, col),
            None => None,
        }
    }

    fn expr_not_in(value: Self::TargetContainer, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_not_in(value, col),
            None => None,
        }
    }

    fn expr_like(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_like(value, col),
            None => None,
        }
    }

    fn expr_nlike(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_nlike(value, col),
            None => None,
        }
    }

    fn expr_null(col: impl IntoColumnRef) -> Option<SimpleExpr> {
        Some(sea_query::Expr::col(col).is_null())
    }

    fn expr_not_null(col: impl IntoColumnRef) -> Option<SimpleExpr> {
        Some(sea_query::Expr::col(col).is_not_null())
    }

    fn expr_between(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_between(value, col),
            _ => None,
        }
    }

    fn expr_nbetween(value: Self::TargetRange, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_nbetween(value, col),
            _ => None,
        }
    }

    fn expr_prefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_prefix(value, col),
            None => None,
        }
    }

    fn expr_nprefix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_nprefix(value, col),
            None => None,
        }
    }

    fn expr_suffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_suffix(value, col),
            None => None,
        }
    }

    fn expr_nsuffix(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_nsuffix(value, col),
            None => None,
        }
    }

    fn expr_contain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_contain(value, col),
            None => None,
        }
    }

    fn expr_ncontain(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_ncontain(value, col),
            None => None,
        }
    }

    fn expr_regex(value: Self::Target, col: impl IntoColumnRef) -> Option<SimpleExpr> {
        match value {
            Some(value) => T::expr_regex(value, col),
            None => None,
        }
    }
}
