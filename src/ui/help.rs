pub fn get_help_docs() -> Vec<Vec<&'static str>> {
    vec![
        vec!["Jump to currently playing album", "a", "General"],
        vec![
            "Jump to currently playing artist's album list",
            "A",
            "General",
        ],
        vec!["Increase volume by 10%", "+", "General"],
        vec!["Decrease volume by 10%", "-", "General"],
        vec!["Skip to next track", "n", "General"],
        vec!["Skip to previous track", "p", "General"],
        vec!["Seek backwards 5 seconds", "<", "General"],
        vec!["Seek forwards 5 seconds", ">", "General"],
        vec!["Toggle shuffle", "<Ctrl+s>", "General"],
        vec!["Copy url to currently playing song", "c", "General"],
        vec!["Copy url to currently playing album", "C", "General"],
        vec!["Cycle repeat mode", "<Ctrl+r>", "General"],
        vec![
            "Move selection left",
            "h | <Left Arrow Key> | <Ctrl+b>",
            "General",
        ],
        vec![
            "Move selection down",
            "j | <Down Arrow Key> | <Ctrl+n>",
            "General",
        ],
        vec![
            "Move selection up",
            "k | <Up Arrow Key> | <Ctrl+p>",
            "General",
        ],
        vec![
            "Move selection right",
            "l | <Right Arrow Key> | <Ctrl+f>",
            "General",
        ],
        vec!["Enter input for search", "/", "General"],
        vec!["Pause/Resume playback", "<Space>", "General"],
        vec!["Enter active mode", "<Enter>", "General"],
        vec![
            "Go back or exit when nowhere left to back to",
            "q",
            "General",
        ],
        vec!["Select device to play music on", "d", "General"],
        vec!["Enter hover mode", "<Esc>", "Selected block"],
        vec!["Save track in list or table", "s", "Selected block"],
        vec![
            "Start playback or enter album/artist/playlist",
            "<Enter>",
            "Selected block",
        ],
        vec!["Delete entire input", "<Ctrl+u>", "Search input"],
        vec!["Search with input text", "<Enter>", "Search input"],
        vec![
            "Move cursor one space left",
            "<Left Arrow Key>",
            "Search input",
        ],
        vec![
            "Move cursor one space right",
            "<Right Arrow Key>",
            "Search input",
        ],
        vec!["Jump to start of input", "<Ctrl+a>", "Search input"],
        vec!["Jump to end of input", "<Ctrl+e>", "Search input"],
        vec![
            "Escape from the input back to hovered block",
            "<Esc>",
            "Search input",
        ],
        vec!["Scroll down to next result page", "<Ctrl+d>", "Pagination"],
        vec![
            "Scroll up to previous result page",
            "<Ctrl+u>",
            "Pagination",
        ],
        vec!["Jump to start of playlist", "<Ctrl+a>", "Pagination"],
        vec!["Jump to end of playlist", "<Ctrl+e>", "Pagination"],
        vec!["Delete saved album", "D", "Library -> Albums"],
        vec!["Delete saved playist", "D", "Playlist"],
        vec!["Follow an artists/playlist", "w", "Search result"],
    ]
}
