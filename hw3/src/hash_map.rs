#[macro_export]

macro_rules! hash_map {
    ( $($key: expr => $v: expr), *) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $v);
            )*
            map
        }
    };
}