use euclid::{Box2D, Point2D};

use iced_native::{
    layout, Background, Color, Element, Hasher, Layout, Length, MouseCursor, Point, Rectangle,
    Size, Widget,
};
use iced_wgpu::{Defaults, Primitive, Renderer};

const POINT_RADIUS: f32 = 2.0;
const LINE_WIDTH: f32 = 1.0;

pub type BoxF32 = Box2D<f32, f32>;
pub type PointF32 = Point2D<f32, f32>;

// TODO is it normal for a widget to have a lifetime? Seems to work fine.
pub struct ScatterPlot<'a> {
    bounds: BoxF32,
    points: &'a [PointF32],
}

impl<'a> ScatterPlot<'a> {
    pub fn new(bounds: BoxF32, points: &'a [PointF32]) -> Self {
        Self { bounds, points }
    }
}

impl<'a, Message> Widget<Message, Renderer> for ScatterPlot<'a> {
    // TODO do I want Fill or Shrink? They seem to behave the same in my example.
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(500.0, 500.0))
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> (Primitive, MouseCursor) {
        // TODO doing all these transformations by hand is hard to read and error-prone; use
        // higher-level linear algebra concepts to make things clearer
        let layout_euclid = BoxF32::new(
            Point2D::new(layout.bounds().x, layout.bounds().y),
            Point2D::new(
                layout.bounds().x + layout.bounds().width,
                layout.bounds().y + layout.bounds().height,
            ),
        );
        let transform_data_to_absolute = |x, y| {
            let center_relative_x = (x - self.bounds.min.x) / self.bounds.size().width;
            // Invert the y axis so that positive points up to match math convention
            let center_relative_y = (y * -1.0 - self.bounds.min.y) / self.bounds.size().height;
            (
                layout_euclid.min.x + center_relative_x * layout_euclid.size().width,
                layout_euclid.min.y + center_relative_y * layout_euclid.size().height
            )
        };
        let transform_data_to_absolute_size = |width, height| {
            let relative_width = width / self.bounds.size().width;
            let relative_height = height / self.bounds.size().height;
            (
                relative_width * layout_euclid.size().width,
                relative_height * layout_euclid.size().height
            )
        };
        let mut primitives: Vec<Primitive> = self
            .points
            .iter()
            .map(|point| {
                let (x, y) = transform_data_to_absolute(point.x, point.y);
                Primitive::Quad {
                    bounds: Rectangle {
                        x: x - POINT_RADIUS,
                        y: y - POINT_RADIUS,
                        width: POINT_RADIUS * 2.0,
                        height: POINT_RADIUS * 2.0,
                    },
                    background: Background::Color(Color::BLACK),
                    border_radius: POINT_RADIUS as u16,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                }
            })
            .collect();

        // Draw the x axis
        primitives.push({
            let (x, y) = transform_data_to_absolute(self.bounds.min.x, 0.0);
            let (width, _height) = transform_data_to_absolute_size(self.bounds.width(), 0.0);
            Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y: y - LINE_WIDTH * 0.5,
                    width,
                    height: LINE_WIDTH,
                },
                background: Background::Color(Color::BLACK),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            }
        });

        // Draw the y axis
        primitives.push({
            let (x, y) = transform_data_to_absolute(0.0, self.bounds.max.y);
            let (_width, height) = transform_data_to_absolute_size(0.0, self.bounds.height());
            Primitive::Quad {
                bounds: Rectangle {
                    x: x - LINE_WIDTH * 0.5,
                    y,
                    width: LINE_WIDTH,
                    height,
                },
                background: Background::Color(Color::BLACK),
                border_radius: 0,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            }
        });
        (
            Primitive::Group {
                primitives,
            },
            MouseCursor::OutOfBounds,
        )
    }

    fn hash_layout(&self, _state: &mut Hasher) {
        // No-op because the size is constant
        // TODO is this correct?
    }
}

impl<'a, Message> Into<Element<'a, Message, Renderer>> for ScatterPlot<'a> {
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}