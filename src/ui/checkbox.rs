use iced::mouse;
use iced::widget::canvas::{self, Cache, Geometry, Path, Stroke};
use iced::widget::{container, Canvas};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Theme};

pub struct BurstRing {
    pub progress: f32, // 0..1 over 500 ms
    pub color: Color,
    cache: Cache,
}

impl BurstRing {
    pub fn new(progress: f32, color: Color) -> Self {
        Self { progress, color, cache: Cache::new() }
    }
}

impl<Message> canvas::Program<Message> for BurstRing {
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
            let p = self.progress.clamp(0.0, 1.0);
            // ease-out
            let eased = 1.0 - (1.0 - p).powi(2);
            let scale = 0.4 + (1.6 - 0.4) * eased;
            let alpha = 0.7 * (1.0 - p);
            let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
            let base_r = bounds.width.min(bounds.height) / 2.0;
            let radius = (base_r * scale).max(0.5);
            let circle = Path::circle(center, radius);
            let mut color = self.color;
            color.a = alpha;
            frame.stroke(
                &circle,
                Stroke::default().with_width(2.0).with_color(color),
            );
        });
        vec![geom]
    }
}

pub fn burst_overlay<'a, Message: 'a>(
    progress: f32,
    color: Color,
    size: f32,
) -> Element<'a, Message> {
    container(
        Canvas::new(BurstRing::new(progress, color))
            .width(Length::Fixed(size))
            .height(Length::Fixed(size)),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .into()
}
