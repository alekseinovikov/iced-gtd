use iced::mouse;
use iced::widget::canvas::{self, Cache, Geometry, Path, Stroke};
use iced::widget::{container, Canvas};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};

use crate::theme::Tokens;

pub struct ProgressRing {
    pub progress: f32, // 0.0..1.0
    pub size: f32,
    pub stroke: f32,
    pub track: Color,
    pub fill: Color,
    cache: Cache,
}

impl ProgressRing {
    pub fn new(progress: f32, size: f32, stroke: f32, track: Color, fill: Color) -> Self {
        Self { progress, size, stroke, track, fill, cache: Cache::new() }
    }
}

impl<Message> canvas::Program<Message> for ProgressRing {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            let radius = (self.size / 2.0) - self.stroke;
            let track = Path::circle(center, radius);
            frame.stroke(
                &track,
                Stroke::default()
                    .with_width(self.stroke)
                    .with_color(self.track),
            );
            if self.progress > 0.0 {
                let p = self.progress.clamp(0.0, 1.0);
                let mut builder = canvas::path::Builder::new();
                let start = -std::f32::consts::FRAC_PI_2;
                let sweep = std::f32::consts::TAU * p;
                builder.move_to(Point::new(
                    center.x + radius * start.cos(),
                    center.y + radius * start.sin(),
                ));
                let end = start + sweep;
                builder.arc(canvas::path::Arc {
                    center,
                    radius,
                    start_angle: iced::Radians(start),
                    end_angle: iced::Radians(end),
                });
                let arc = builder.build();
                frame.stroke(
                    &arc,
                    Stroke::default()
                        .with_width(self.stroke)
                        .with_color(self.fill),
                );
            }
        });
        vec![geom]
    }
}

pub fn ring<'a, Message: 'a>(progress: f32, active: bool, tokens: Tokens) -> Element<'a, Message> {
    let track = if active { tokens.accent } else { tokens.ink_4 };
    let fill = if active { tokens.accent } else { tokens.ink_3 };
    let prog = ProgressRing::new(progress.clamp(0.0, 1.0), 12.0, 1.6, track, fill);
    container(
        Canvas::new(prog)
            .width(Length::Fixed(12.0))
            .height(Length::Fixed(12.0)),
    )
    .width(Length::Fixed(12.0))
    .height(Length::Fixed(12.0))
    .into()
}
