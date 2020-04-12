extern crate iced_plot;

use iced::{button, Align, Button, Column, Container, Element, Length, Sandbox, Settings, Text};
use rand::{thread_rng, RngCore};
use iced_plot::ScatterPlot;

pub fn main() {
    Example::run(Settings::default())
}

struct Example {
    rng: Box<dyn RngCore>,
    points: Vec<Point2D<f32, f32>>,
    button_state: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
}

use euclid::Box2D;
use euclid::Point2D;
use rand::distributions::Distribution;

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            rng: Box::new(thread_rng()),
            points: Vec::new(),
            button_state: button::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Gaussian plot widget example in Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                // TODO oops this is deprecated, need to use the rand_distr crate
                let normal = rand::distributions::Normal::new(0.0, 1.0);
                self.points.push(Point2D::new(
                    normal.sample(&mut self.rng) as f32,
                    normal.sample(&mut self.rng) as f32,
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
                Box2D {
                    min: Point2D::new(-5.0, -5.0),
                    max: Point2D::new(5.0, 5.0),
                },
                &self.points,
            ))
            .push(Text::new(format!("Point count: {}", self.points.len())))
            .push(
                Button::new(&mut self.button_state, Text::new("Add a random point"))
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
