#[macro_export]
macro_rules! query {
    (from $db:ident select $field:ident) => {
        $db.into_iter().map(|i| i.$field).collect()
    };
}
