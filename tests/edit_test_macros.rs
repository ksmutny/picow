#[macro_export]
macro_rules! edit_test {
    ($name:ident: $($line_in:expr),*; $($event:expr),*; $($line_exp:expr),*) => {
        #[test]
        fn $name() {
            let mut state = state(vec![$($line_in),*]);
            let events = vec![$($event),*];

            events.into_iter().for_each(|event| {
                process_event(&event, &mut state);
            });

            assert(&state, vec![$($line_exp),*]);
        }
    };
}

#[macro_export]
macro_rules! s { ($x:expr) => ($x.to_string()); }

#[macro_export]
macro_rules! vecr { ($($x:expr),*) => (vec![$(Row::new($x)),*]); }
