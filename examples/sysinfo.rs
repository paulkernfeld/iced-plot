extern crate iced_plot;

use euclid::{Box2D, Point2D};
use iced::{button, Align, Button, Column, Container, Element, Length, Sandbox, Settings, Text};
use iced_plot::ScatterPlot;
use std::time::Instant;
use sysinfo::{System, SystemExt as _};

struct Example {
    points: Vec<Point2D<f32, f32>>,
    button_state: button::State,
    t0: Instant,
    system: System,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            points: Vec::new(),
            button_state: button::State::new(),
            t0: Instant::now(),
            system: System::new_all(),
        }
    }

    fn title(&self) -> String {
        String::from("Available memory plot widget example in Iced")
    }

    fn update(&mut self, message: Message) {
        // TODO only refresh memory, not other system info
        self.system.refresh_all();

        match message {
            Message::ButtonPressed => {
                self.points.push(Point2D::new(
                    self.t0.elapsed().as_secs_f32(),
                    self.system.get_used_swap() as f32,
                ));
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Align::Center)
            .push(ScatterPlot::new(
                // TODO it would be nice to adjust the timescale dynamically to the available data
                Box2D {
                    min: Point2D::new(0.0, 0.0),
                    max: Point2D::new(30.0, self.system.get_total_memory() as f32),
                },
                &self.points,
            ))
            .push(
                // TODO refresh the memory automatically on a timer. I think this requires that
                // Example implement Application instead of Sandbox so that it can receive async
                // events.
                Button::new(&mut self.button_state, Text::new("Refresh memory"))
                    .on_press(Message::ButtonPressed),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

pub fn main() {
    Example::run(Settings::default())
}
