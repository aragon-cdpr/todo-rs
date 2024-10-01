mod todo;
use todo::Todo;

use std::io::{stdout, Result};

use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, layout::*, style::{Color, Modifier, Style, Stylize}, widgets::*, DefaultTerminal, Frame
};

struct CustomColor;

impl CustomColor{
    const GOLD: Color = Color::Rgb(208, 164, 92);
    const WHITE: Color = Color::Rgb(218, 225, 229);
    const RED: Color = Color::Rgb(255, 31, 48);
    const GREY: Color = Color::Rgb(57, 60, 71);
}

enum AppState {
    Default,
    Add,
    Remove
}

struct UserInput {
    input: String,
    char_index: usize
}
impl UserInput {
    fn new() -> UserInput {
        UserInput {
            input: String::new(),
            char_index: 0
        }
    }

    fn reset(&mut self){
            self.input = String::new();
            self.char_index = 0;
            self.reset_cursor();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.char_index.saturating_sub(1);
        self.char_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.char_index.saturating_add(1);
        self.char_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, char: char) {
        let index = self.byte_index();
        self.input.insert(index, char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.char_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.char_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.char_index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.char_index = 0;
    }

}


fn main() -> Result<()> {
    //TODO: implement db
    //TODO: get the todos from db
    let mut todos: Vec<Todo> = vec![
        Todo::new(
            String::from("windows doesn't suck(lie)"), 
            false
        ),
        Todo::new(
            String::from("learn rust"), 
            false
        )
    ];

    //TODO: clean the mess in the code, and seperate to different files
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let mut state = ListState::default().with_selected(Some(0));
    let mut app_state: AppState = AppState::Default;
    let mut input = UserInput::new();

    loop {
        render_ui(&mut terminal, &todos, &mut state, &app_state, &input)?;
        //NOTE: Event handling
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app_state {
                        AppState::Default => match key.code {
                            KeyCode::Down => state.scroll_down_by(1),
                            KeyCode::Up => state.scroll_up_by(1),
                            KeyCode::Enter => {
                                let index = state.selected().unwrap();
                                todos[index].toggle();
                            },
                            KeyCode::Char('a') => app_state = AppState::Add,
                            KeyCode::Char('d') => app_state = AppState::Remove,
                            KeyCode::Esc => continue,
                            KeyCode::Char('q') | _ => break
                        },
                        AppState::Add => match key.code {
                            KeyCode::Esc => {
                                input.reset();
                                app_state = AppState::Default;
                            },
                            KeyCode::Char(to_insert) => input.enter_char(to_insert),
                            KeyCode::Backspace => input.delete_char(),
                            KeyCode::Enter => {
                                let todo = Todo::new(input.input.clone(), false);
                                input.reset();
                                todos.push(todo);
                                app_state = AppState::Default;
                            },
                            _ => app_state = AppState::Default,
                        },
                        AppState::Remove => {
                            //TODO: render widget to ask user if he wants to delete item
                            //TODO: refresh listing on demand to update list immediately
                            let index = state.selected().unwrap();
                            todos.remove(index);
                            app_state = AppState::Default;
                        }
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render_ui(terminal: &mut DefaultTerminal, list: &Vec<Todo>,  state: &mut ListState, app_state: &AppState, input: &UserInput) -> Result<()> {
    terminal.draw(|frame| {
        let area = frame.area();
        //NOTE: Default app look
        frame.render_widget(Clear, area);
        frame.render_stateful_widget(
            List::new(list)
                .bg(CustomColor::GREY)
                .fg(CustomColor::WHITE)
                .highlight_style(Style::new().add_modifier(Modifier::UNDERLINED).fg(CustomColor::GREY).bg(CustomColor::GOLD))
                .highlight_symbol(">>"),
            area, state
        );
        match app_state {
            AppState::Add => render_add_widget(frame, area, input),
            _ => {}
        }
    })?;
    Ok(())
}

fn render_add_widget(frame: &mut Frame<'_>, area: Rect, input: &UserInput) {
    let paragraph = 
        Paragraph::new(input.input.as_str())
            .left_aligned()
            .bg(CustomColor::GREY)
            .wrap(Wrap::default())
            .block(
                Block::bordered()
                .title("Add item")
                .fg(CustomColor::GOLD)
            )
            .add_modifier(Modifier::RAPID_BLINK);
        //TODO: cursor is not blinking while popup is up
        //TODO: maybe add some shadow to the block to differentiate from the background

    let v = Layout::vertical([Constraint::Percentage(20)]).flex(Flex::Center);
    let h = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
    let [area] = v.areas(area);
    let [area]  = h.areas(area); 
    frame.render_widget(Clear, area);
    frame.set_cursor_position(Position::new(
        area.x + input.char_index as u16 + 1,
        area.y + 1,
    ));
    frame.render_widget(paragraph, area);
}

fn render_remove_widget() {
    todo!();
    //TODO: implement remove widget feature
}
