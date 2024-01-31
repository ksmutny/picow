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
      // 12345678901234
        "┌─────▯─────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );


    test_nav!(
        move_up_3_from_line_4
        move_up(3);
      // 12345678901234
        "┌─────▯─────┐",
        "│_____      │",
        "│______     │",
        "└─────▮─────┘"
    );

    test_nav!(
        move_up_3_from_line_2
        move_up(3);
      // 12345678901234
        "╔_____▯__    ",
        "_____        ",
        "┌───────────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_up_to_shorter_line
        move_up(1);
      // 12345678901234
        "┌───────────┐",
        "│___▯       │",
        "│______▮    │",
        "└───────────┘"
    );
}
