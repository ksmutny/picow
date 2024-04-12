#[macro_export]
macro_rules! s { ($x:expr) => ($x.to_string()); }

#[macro_export]
macro_rules! vecs { ($($x:expr),*) => (vec![$(s!($x)),*]); }
