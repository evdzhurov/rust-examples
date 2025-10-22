// Note: this macro borrows db and clones each field.
// Not sure if this is a good design overall

#[macro_export]
macro_rules! query {
    (from $db:ident select $($field:ident),+) => {
        $db.iter().map(|i| ($(i.$field.clone(),)+)).collect()
    };

    (from $db:ident select $($field:ident),+ where $($test_field:ident = $value:literal) and +) => {
        $db.iter()
            .filter( |i| $(i.$test_field == $value)&&+)
            .map(|i| ($(i.$field.clone(),)+)).collect()
    };
}
