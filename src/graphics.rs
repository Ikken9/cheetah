extern crate alloc;
extern crate esp_hal;

use core::convert::Infallible;

use embedded_graphics_core::draw_target::DrawTarget;
use embedded_graphics_core::geometry::{Dimensions, Size};
use embedded_graphics_core::Pixel;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::Point;
use embedded_graphics_core::primitives::{PointsIter, Rectangle};

use crate::display::ILI9488;

impl Dimensions for ILI9488<'_> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(320,480))
    }
}

impl DrawTarget for ILI9488<'_> {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
        where I: IntoIterator<Item=Pixel<Self::Color>>
    {
        for Pixel(coord, color) in pixels {
            if self.bounding_box().contains(coord) {
                let x = coord.x as u16;
                let y = coord.y as u16;
                self.draw_pixel(x, y, color);
            }
        }
        Ok(())
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
        where I: IntoIterator<Item=Self::Color>
    {
        let drawable_area = area.intersection(&self.bounding_box());
        if let Some(_) = drawable_area.bottom_right() {
            if area == &drawable_area {
                self.draw_contiguous(area.points()
                    .zip(colors)
                )
            } else {
                self.draw_contiguous(area.points()
                    .zip(colors)
                    .filter(|(point, _)| drawable_area.contains(*point))
                )
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let drawable_area = area.intersection(&self.bounding_box());
        if let Some(_) = drawable_area.bottom_right() {
            if area == &drawable_area {
                self.fill(area.points(), color)
            } else {
                self.fill(area.points()
                    .filter(|point| drawable_area.contains(*point)), color)
            }
        }
        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear(color);
        Ok(())
    }
}