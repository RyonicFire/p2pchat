use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;

use anathema::runtime::{KeyCode, KeyEvent, Runtime};
use anathema::templates::DataCtx;
use anathema::widgets::Value;

use crate::commandparser::parse;
use crate::connections::ConnectionMsg;

pub use anathema::runtime::Event;
pub use anathema::display::Color;
pub type TuiEventSender = mpsc::Sender<Event<StdoutMsg>>;

pub struct StdoutMsg {
    pub msg: String,
    pub foreground: Color,
    pub background: Color,
}

impl From<StdoutMsg> for Value {
    fn from(value: StdoutMsg) -> Self {
        let mut hm = HashMap::new();
        hm.insert("msg".to_string(), value.msg.into());
        hm.insert("foreground".to_string(), value.foreground.into());
        hm.insert("background".to_string(), value.background.into());

        Self::Map(hm)
    }
}

impl StdoutMsg {
    pub fn new(msg: String) -> Self {
        Self {
            msg,
            foreground: Color::Reset,
            background: Color::Reset
        }
    }

    pub fn with_foreground(msg: String, foreground: Color) -> Self {
        Self {
            msg,
            foreground,
            background: Color::Reset
        }
    }

    pub fn with_background(msg: String, background: Color) -> Self {
        Self {
            msg,
            foreground: Color::Reset,
            background
        }
    }

    pub fn with_color(msg: String, foreground: Color, background: Color) -> Self {
        Self {
            msg,
            foreground,
            background
        }
    }
}

pub struct Tui {
    runtime: Runtime<StdoutMsg>,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new(),
        }
    }

    pub fn sender(&self) -> mpsc::Sender<Event<StdoutMsg>> {
        self.runtime.tx()
    }

    pub fn start(self, con_sender: mpsc::Sender<ConnectionMsg>, username: &str) {
        let template =
            fs::read_to_string("./templates/main.tiny").expect("Cant read template.tiny");
        let mut ctx = DataCtx::empty();
        let messages: Vec<StdoutMsg> = Vec::new();
        ctx.insert("input", "");
        ctx.insert("messages", messages);

        self.runtime.start(template, ctx, |event, _root, ctx, tx| {
            if event.ctrl_c() {
                tx.send(Event::Quit).unwrap();
            }

            if let Event::Key(KeyEvent { code, .. }) = event {
                let input = ctx.get_string_mut("input").expect("No input found in context");
                match code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Enter => {
                        match parse(input, username.as_bytes().to_vec(), tx.clone()) {
                            Ok(Some(msg)) => {
                                if let Err(err) = con_sender.send(msg) {
                                    tx.send(Event::User(StdoutMsg::new(format!(
                                        "Error while sending message ton connection handler via mpsc: {err}"
                                    ))));
                                    return;
                                }
                            }
                            Ok(None) => return,
                            Err(e) => {
                                tx.send(Event::User(StdoutMsg::new(format!(
                                    "{e}"
                                ))));
                                return;
                            }
                        }
                        input.clear();
                    }
                    _ => return
                }
            }

            if let Event::User(msg) = event {
                let value = ctx.get_mut("messages").expect("No messages found in context");
                if let Value::List(messages) = value {
                    messages.push(msg.into());
                }
            }
        }).unwrap();
    }
}