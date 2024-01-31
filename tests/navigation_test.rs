#[macro_use]
#[path ="./navigation_macros.rs"]
mod navigation_macros;

#[path ="./parse_editor.rs"]
mod parse_editor;

mod move_up {
    use super::parse_editor::parse_test_case;
    use picow::editor::navigation::*;

    test_nav!(
        move_up_1_from_line_2
        move_up(1);
        "┌─────▯─────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );


    test_nav!(
        move_up_3_from_line_4
        move_up(3);
        "┌─────▯─────┐",
        "│_____      │",
        "│______     │",
        "└─────▮─────┘"
    );

    test_nav!(
        move_up_3_from_line_2
        move_up(3);
        "╔_____▯__    ",
        "_____        ",
        "┌───────────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );
}
