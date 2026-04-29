use iced::mouse;
use iced::widget::canvas::{self, Cache, Geometry, Path, Stroke};
use iced::widget::{Canvas, container};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};

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
        Self {
            progress,
            size,
            stroke,
            track,
            fill,
            cache: Cache::new(),
        }
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

            let track_path = Path::circle(center, radius);
            frame.stroke(
                &track_path,
                Stroke::default()
                    .with_width(self.stroke)
                    .with_color(self.track),
            );

            let p = self.progress.clamp(0.0, 1.0);
            if p > 0.0 {
                let arc = Path::new(|builder| {
                    let segments = 48;
                    let start = -std::f32::consts::FRAC_PI_2;
                    let end = start + std::f32::consts::TAU * p;
                    for i in 0..=segments {
                        let t = i as f32 / segments as f32;
                        let a = start + (end - start) * t;
                        let pt =
                            Point::new(center.x + radius * a.cos(), center.y + radius * a.sin());
                        if i == 0 {
                            builder.move_to(pt);
                        } else {
                            builder.line_to(pt);
                        }
                    }
                });
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

pub fn ring<'a, Message: 'a>(
    progress: f32,
    active: bool,
    tokens: crate::theme::Tokens,
) -> Element<'a, Message> {
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
