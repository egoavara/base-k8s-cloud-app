use table_traits_derive::filter_internal;
use uuid::Uuid;

filter_internal! {
    pub struct DefaultUuidFilter for Uuid impl eq + in;
    pub struct DefaultStringFilter for String impl eq + ne + in + nin + prefix + nprefix + contains + ncontains + suffix + nsuffix + like + nlike + regex;
}
