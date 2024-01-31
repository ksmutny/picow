#[macro_use]
#[path ="./navigation_macros.rs"]
mod navigation_macros;

#[path ="./parse_editor.rs"]
mod parse_editor;

mod move_up {
    use super::parse_editor::parse_test_case;
    use picow::editor::navigation::move_up;

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
        move_to_shorter_line
        move_up(1);
      // 12345678901234
        "┌───────────┐",
        "│___▯       │",
        "│______▮    │",
        "└───────────┘"
    );
}


mod move_down {
    use super::parse_editor::parse_test_case;
    use picow::editor::navigation::move_down;

    test_nav!(
        move_to_scree_bottom
        move_down(3);
      // 12345678901234
        "┌─────▮─────┐",
        "│_____      │",
        "│_____      │",
        "└─────▯─────┘"
    );

    test_nav!(
        scroll_down
        move_down(4);
      // 12345678901234
        "┌─────▮─────┐",
        "╔_____      │",
        "│_____      │",
        "└───────────┘",
        "______▯____  "
    );

    test_nav!(
        move_to_shorter_line
        move_down(1);
      // 12345678901234
        "┌───────────┐",
        "│______▮    │",
        "│___▯       │",
        "└───────────┘"
    );

    test_nav!(
        move_to_eof
        move_down(5);
      // 12345678901234
        "┌───────────┐",
        "│______▮    │",
        "│__☼▯       │",
        "│           │",
        "│           │",
        "│           │",
        "└───────────┘"
    );
}
