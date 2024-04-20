#[macro_export]
macro_rules! s { ($x:expr) => ($x.to_string()); }

#[macro_export]
macro_rules! vecr { ($($x:expr),*) => (vec![$(Row::new($x)),*]); }
