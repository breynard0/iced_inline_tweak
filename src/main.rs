use iced::{alignment::Horizontal, widget::*, Element, Length};

use inline_tweak::*;

fn main() {
    iced::run("Iced Inline Tweak", State::update, State::view).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    CounterUp,
    CounterDown,
}

#[derive(Debug, Default)]
struct State {
    value: i32,
}

impl State {
    // This library has function-like procedural macros that allow tweaking
    fn update(&mut self, message: Message) {
        use std::column; // Something in the macro conflicts with Iced's column type, so a use state must be used
        match message {
            Message::CounterUp => self.value += tweak!(10),
            Message::CounterDown => self.value -= tweak!(1),
        }
    }
    
    // The library also has a derive macro that enables all literals in a function to be tweaked
    #[tweak_fn]
    fn view(&self) -> Element<Message> {
        let title_text = text("Counter!")
            .size(32.0)
            .color([1.0, 0.0, 0.0]);
        
        let up_button = button(text("Up")).on_press(Message::CounterUp).width(Length::FillPortion(3));
        let down_button = button(text("Down")).on_press(Message::CounterDown).width(Length::FillPortion(3));
        let counter_text = text(self.value.to_string()).width(Length::FillPortion(3)).size(40.0).align_x(Horizontal::Center);
        let row = row!(up_button, counter_text, down_button);
        
        iced::widget::column![
            title_text,
            row
        ].into()
    }
}