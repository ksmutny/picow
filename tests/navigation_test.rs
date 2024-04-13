#[macro_use]
#[path ="./navigation_macros.rs"]
mod navigation_macros;

#[path ="./parse_editor_state.rs"]
mod parse_editor_state;

mod move_up {
    use picow::editor::cursor::Cursor;
    use super::parse_editor_state::parse_test_case;

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
    use picow::editor::cursor::Cursor;
    use super::parse_editor_state::parse_test_case;

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
        move_to_shorter_line_scroll_left
        move_down(1);
        "______┌───────────┐",
        "_╔____│___        │",
        "______│______     │",
        "______└─────────▮─┘",
        "_▯                 "
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

mod document_start_end {
    use picow::editor::cursor::Cursor;
    use super::parse_editor_state::parse_test_case;

    test_nav!(
        move_to_document_start
        move_document_start();
      // 12345678901234
        "▯_______    ",
        "_____        ",
        "┌───────────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_to_document_end
        move_document_end();
      // 12345678901234
        "┌───────────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘",
        "_______      ",
        "_______      ",
        "╔______      ",
        "_______      ",
        "_______      ",
        "___☼▯        "
    );
}

mod move_horizoval {
    use picow::editor::cursor::Cursor;
    use super::parse_editor_state::parse_test_case;

    test_nav!(
        move_to_line_start
        move_line_start();
        "┌───────────┐",
        "▯_____▮     │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_to_line_start_no_move
        move_line_start();
        "┌───────────┐",
        "▮_____      │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_to_line_send
        move_line_end();
        "┌───────────┐",
        "│__▮__▯     │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_left_within_line
        move_left();
        "┌───────────┐",
        "│__▯▮__     │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_left_prev_line
        move_left();
        "┌───────────┐",
        "│____▯      │",
        "▮______     │",
        "└───────────┘"
    );

    test_nav!(
        move_left_scroll_left
        move_left();
        "___________   ",
        "╔┌───────────┐",
        "▯▮____       │",
        "_│______     │",
        "_└───────────┘"
    );

    test_nav!(
        move_left_prev_line_scroll_up
        move_left();
        "╔________▯   ",
        "▮───────────┐",
        "│____       │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_left_document_start
        move_left();
        "▮───────────┐",
        "│____       │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_right_within_line
        move_right();
        "┌───────────┐",
        "│__▮▯__     │",
        "│______     │",
        "└───────────┘"
    );

    test_nav!(
        move_right_next_line
        move_right();
        "┌───────────┐",
        "│____.▮     │",
        "▯______     │",
        "└───────────┘"
    );

    test_nav!(
        move_right_scroll_right
        move_right();
        "______________________",
        "_┌╔──────────┐____    ",
        "_│____       │        ",
        "_│___________▮▯___    ",
        "_└───────────┘        "
    );

    test_nav!(
        move_right_next_line_scroll
        move_right();
        "┌───────────┐",
        "╔____       │",
        "│____       │",
        "└───────.▮  ┘",
        "▯______     │"
    );

    test_nav!(
        move_right_document_end
        move_right();
        "┌───────────┐",
        "│____☼▮     │",
        "│           │",
        "└───────────┘"
    );
}
