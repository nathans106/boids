use iced::widget::canvas;
use iced::{
    executor, window, Command, Element, Length, Point, Rectangle, Renderer, Size, Subscription,
    Theme,
};
use std::alloc::Global;
use std::time::Instant;

use crate::simulation::Simulation;

const BOID_SIZE: Size = Size::new(5.0, 5.0);

pub struct App {
    simulation: Simulation,
    cache: canvas::Cache,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Tick(Instant),
}

impl iced::Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                simulation: Simulation::default(),
                cache: canvas::Cache::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Boids")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick(_instant) => {
                self.cache.clear();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        canvas(self as &Self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        window::frames().map(Message::Tick)
    }
}

impl<Message> canvas::Program<Message> for App {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle<f32>,
        _cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry, Global> {
        let cache = self.cache.draw(bounds.size(), |frame| {
            for boid in self.simulation.boids().iter() {
                let (x, y) = *boid.coord();
                let point = Point::new(x, y);
                let path = canvas::Path::rectangle(point, BOID_SIZE);
                let stroke = canvas::Stroke::default();
                frame.stroke(&path, stroke);
            }
        });

        vec![cache]
    }
}
