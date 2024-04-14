#[macro_use]
#[path ="./editor_macros.rs"]
mod editor_macros;

#[path ="./parse_editor.rs"]
mod parse_editor;

mod move_up {
    use picow::terminal::events::{Event::*,KeyCode::*};
    use super::parse_editor::parse_test_case;

    test_editor!(
        move_up_1_from_line_2
        Key(Up, 0);
      // 1234567890123
        "┌─────▯─────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );


    test_editor!(
        move_up_3_from_line_4
        Key(Up, 0),
        Key(Up, 0),
        Key(Up, 0);
      // 1234567890123
        "┌─────▯─────┐",
        "│_____      │",
        "│______     │",
        "└─────▮─────┘"
    );

    test_editor!(
        move_up_3_from_line_2
        Key(Up, 0),
        Key(Up, 0),
        Key(Up, 0);
      // 1234567890123
        "╔_____▯__    ",
        "_____        ",
        "┌───────────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_to_shorter_line
        Key(Up, 0);
      // 1234567890123
        "┌───────────┐",
        "│___▯       │",
        "│______▮    │",
        "└───────────┘"
    );
}


mod move_down {
    use picow::terminal::events::{Event::*,KeyCode::*};
    use super::parse_editor::parse_test_case;

    test_editor!(
        move_to_scree_bottom
        Key(Down, 0),
        Key(Down, 0),
        Key(Down, 0);
      // 1234567890123
        "┌─────▮─────┐",
        "│_____      │",
        "│_____      │",
        "└─────▯─────┘"
    );

    test_editor!(
        scroll_down
        Key(Down, 0),
        Key(Down, 0),
        Key(Down, 0),
        Key(Down, 0);
      // 12345678901234
        "┌─────▮─────┐",
        "╔_____      │",
        "│_____      │",
        "└───────────┘",
        "______▯____  "
    );

    test_editor!(
        move_to_shorter_line
        Key(Down, 0);
      // 1234567890123
        "┌───────────┐",
        "│______▮    │",
        "│___▯       │",
        "└───────────┘"
    );

    test_editor!(
        move_to_shorter_line_scroll_left
        Key(Down, 0);
        "______┌───────────┐",
        "_╔____│___        │",
        "______│______     │",
        "______└─────────▮─┘",
        "_▯                 "
    );

    test_editor!(
        move_to_eof
        Key(Down, 0),
        Key(Down, 0),
        Key(Down, 0),
        Key(Down, 0),
        Key(Down, 0);
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
    use picow::terminal::events::{Event::*,KeyCode::*,CTRL};
    use super::parse_editor::parse_test_case;

    test_editor!(
        move_to_document_start
        Key(Home, CTRL);
      // 1234567890123
        "▯_______    ",
        "_____        ",
        "┌───────────┐",
        "│_____▮     │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_to_document_end
        Key(End, CTRL);
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
    use picow::terminal::events::{Event::*,KeyCode::*};
    use super::parse_editor::parse_test_case;

    test_editor!(
        move_to_line_start
        Key(Home, 0);
        "┌───────────┐",
        "▯_____▮     │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_to_line_start_no_move
        Key(Home, 0);
        "┌───────────┐",
        "▮_____      │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_to_line_send
        Key(End, 0);
        "┌───────────┐",
        "│__▮__▯     │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_left_within_line
        Key(Left, 0);
        "┌───────────┐",
        "│__▯▮__     │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_left_prev_line
        Key(Left, 0);
        "┌───────────┐",
        "│____▯      │",
        "▮______     │",
        "└───────────┘"
    );

    test_editor!(
        move_left_scroll_left
        Key(Left, 0);
        "___________   ",
        "╔┌───────────┐",
        "▯▮____       │",
        "_│______     │",
        "_└───────────┘"
    );

    test_editor!(
        move_left_prev_line_scroll_up
        Key(Left, 0);
        "╔________▯   ",
        "▮───────────┐",
        "│____       │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_left_document_start
        Key(Left, 0);
        "▮───────────┐",
        "│____       │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_right_within_line
        Key(Right, 0);
        "┌───────────┐",
        "│__▮▯__     │",
        "│______     │",
        "└───────────┘"
    );

    test_editor!(
        move_right_next_line
        Key(Right, 0);
        "┌───────────┐",
        "│____.▮     │",
        "▯______     │",
        "└───────────┘"
    );

    test_editor!(
        move_right_scroll_right
        Key(Right, 0);
        "______________________",
        "_┌╔──────────┐____    ",
        "_│____       │        ",
        "_│___________▮▯___    ",
        "_└───────────┘        "
    );

    test_editor!(
        move_right_next_line_scroll
        Key(Right, 0);
        "┌───────────┐",
        "╔____       │",
        "│____       │",
        "└───────.▮  ┘",
        "▯______     │"
    );

    test_editor!(
        move_right_document_end
        Key(Right, 0);
        "┌───────────┐",
        "│____☼▮     │",
        "│           │",
        "└───────────┘"
    );
}
