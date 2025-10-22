// Note: this macro borrows db and clones each field.
// Not sure if this is a good design overall

#[macro_export]
macro_rules! query {
    (from $db:ident select $($field:ident),+) => {
        $db.iter().map(|i| ($(i.$field.clone(),)+)).collect()
    };

    (from $db:ident select $($field:ident),+ where $($where_tree:tt)+) => {
        $db.iter()
            .filter( |i| where_clause!(i; $($where_tree)+))
            .map(|i| ($(i.$field.clone(),)+)).collect()
    };
}

#[macro_export]
macro_rules! where_clause {
    ( $i:ident; $test_field:ident $comp:tt $value:literal) => {
        $i.$test_field $comp $value
    };

    ($i:ident; $test_field:ident $comp:tt $value:literal and $($tail:tt)+) => {
        $i.$test_field $comp $value && where_clause!($i; $($tail)+)
    };

    ($i:ident; $test_field:ident $comp:tt $value:literal or $($tail:tt)+) => {
        $i.$test_field $comp $value || where_clause!($i; $($tail)+)
    };
}
