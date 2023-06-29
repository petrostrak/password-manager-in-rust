use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph};

const APP_KEYS_DESC: &str = r#"
L:           List
U:           On list, It's copy the Username
P:           On list, It's copy the Password
D:           On list, It's Delete
E:           On list, It's Edit
S:           Search
Insert Btn:  Insert new Password
Tab:         Go to next field
Shift+Tab:   Go to previous filed
Esc:         Exit insert mode
"#;

enum InputMode {
    Normal,
    Title,
    Username,
    Password,
    Submit,
    Search,
    List,
}

struct Password {
    title: String,
    username: String,
    password: String,
}

struct PassMng {
    mode: InputMode,
    list_state: ListState,
    passwords: Vec<Password>,
    search_text: String,
    search_list: Vec<Password>,
    new_title: String,
    new_username: String,
    new_password: String,
}

fn main() {
    println!("Hello, world!");
}
