use termion::event::{Event, Key};

use crate::commands::cursor_move;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{CommandKeybind, NumberedExecute};
use crate::ui::views::TuiView;
use crate::ui::AppBackend;

pub fn numbered_command(
    first_char: char,
    context: &mut AppContext,
    backend: &mut AppBackend,
    keymap: &AppKeyMapping,
) -> JoshutoResult {
    context.flush_event();
    let mut prefix = String::from(first_char);

    loop {
        context.message_queue_mut().push_info(prefix.clone());
        backend.render(TuiView::new(context));
        context.message_queue_mut().pop_front();

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        let num_prefix = match prefix.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                context.message_queue_mut().pop_front();
                return Err(JoshutoError::new(
                    JoshutoErrorKind::ParseError,
                    "Number is too big".to_string(),
                ));
            }
        };

        match event {
            AppEvent::Termion(event) => {
                match event {
                    Event::Key(Key::Esc) => return Ok(()),
                    Event::Key(Key::Char('g')) => {
                        cursor_move::cursor_move(context, num_prefix - 1);
                        return Ok(());
                    }
                    Event::Key(Key::Char(c)) if c.is_numeric() => {
                        prefix.push(c);
                    }
                    key => {
                        match keymap.default_view.get(&key) {
                            Some(CommandKeybind::SimpleKeybind(filetypes)) => {
                                let command = context
                                    .tab_context_ref()
                                    .curr_tab_ref()
                                    .curr_list_ref()
                                    .and_then(|s| s.curr_entry_ref())
                                    .map(|entry| entry.metadata.file_type())
                                    .and_then(|file_type| filetypes.get(&Some(*file_type)))
                                    .or_else(|| filetypes.get(&None));
                                match command {
                                    Some(command) => {
                                        return command
                                            .numbered_execute(num_prefix, context, backend, keymap)
                                    }
                                    _ => return Err(JoshutoError::new(
                                        JoshutoErrorKind::UnrecognizedCommand,
                                        "Command cannot be prefixed by a number or does not exist"
                                            .to_string(),
                                    )),
                                }
                            }
                            _ => {
                                return Err(JoshutoError::new(
                                    JoshutoErrorKind::UnrecognizedCommand,
                                    "Command cannot be prefixed by a number or does not exist"
                                        .to_string(),
                                ));
                            }
                        }
                    }
                }
                context.flush_event();
            }
            event => process_event::process_noninteractive(event, context),
        }
    }
}
