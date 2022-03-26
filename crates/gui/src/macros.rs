macro_rules! unwrap_or_return {
    ( $e:expr, $a: expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err(String::from($a)),
        }
    };
}

pub(crate) use unwrap_or_return;
