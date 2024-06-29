use alloc::rc::Rc;
use core::cell::RefCell;
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::ascii::{FONT_7X14, FONT_9X15};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::primitives::{Primitive, PrimitiveStyle};
use embedded_graphics::text::Text;
use embedded_graphics::transform::Transform;
use embedded_graphics_core::Drawable;
use embedded_graphics_core::geometry::{Point, Size};
use embedded_graphics_core::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics_core::primitives::Rectangle;
use tinybmp::Bmp;

use crate::display::ILI9488;

type Link<F> = Option<Rc<RefCell<Level<F>>>>;

pub struct Menu<'a, F> {
    pub(crate) head: Link<F>,
    current: Link<F>,
    display: &'a mut ILI9488<'a>,
    cursor_shape: Image<'a, Bmp<'a, Rgb888>>,
    cursor_x: i32,
    cursor_y: i32
}

impl<'a, F> Menu<'a, F> {
    pub fn new<>(display: &'a mut ILI9488<'a>, bmp: &'a Bmp<'a, Rgb888>, x: i32, y: i32) -> Menu<'a, F> {
        let menu = Menu {
            head: None,
            current: None,
            display,
            cursor_shape: Image::new(bmp, Point::new(x, y)),
            cursor_x: x,
            cursor_y: y
        };

        menu.cursor_shape.draw(menu.display).unwrap();
        menu
    }

    pub fn append_horizontal(&mut self, node: Rc<RefCell<Level<F>>>) {
        let mut last = self.head.clone();
        while let Some(next) = last.as_ref().and_then(|node| node.borrow().next.clone()) {
            last = Some(next);
        }
        if let Some(last) = last {
            last.borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = Some(last);
        } else {
            self.head = Some(node.clone());
        }
        if self.current.is_none() {
            self.current = Some(node);  // Automatically set focus to the first node added
        }
    }

    pub fn append_vertical(&mut self, node: Rc<RefCell<Level<F>>>) {
        let mut last = self.head.clone();
        while let Some(down) = last.as_ref().and_then(|node| node.borrow().down.clone()) {
            last = Some(down);
        }
        if let Some(last) = last {
            last.borrow_mut().down = Some(node.clone());
            node.borrow_mut().up = Some(last);
        } else {
            self.head = Some(node.clone());
        }
        if self.current.is_none() {
            self.current = Some(node);
        }
    }

    pub fn move_ok(&mut self) {
        let next = self.current.as_ref().and_then(|current| current.borrow().next.clone());
        if let Some(next) = next {
            self.current = Some(next.clone());
            if let Some(draw) = next.borrow().draw {
                draw(self.display)
            }
        }
    }

    pub fn move_return(&mut self) {
        let prev = self.current.as_ref().and_then(|current| current.borrow().prev.clone());
        if let Some(prev) = prev {
            self.current = Some(prev.clone());
            if let Some(draw) = prev.borrow().draw {
                draw(self.display)
            }
        }
    }

    pub fn move_up(&mut self) {
        let up = self.current.as_ref().and_then(|current| current.borrow().up.clone());
        if let Some(up) = up {
            self.current = Some(up.clone());
            self.update_position(
                self.current.clone().unwrap().borrow().x,
                self.current.clone().unwrap().borrow().y);
        }
    }

    pub fn move_down(&mut self) {
        let down = self.current.as_ref().and_then(|current| current.borrow().down.clone());
        if let Some(down) = down {
            self.current = Some(down.clone());
            self.update_position(
                self.current.clone().unwrap().borrow().x,
                self.current.clone().unwrap().borrow().y);
        }
    }

    fn update_position(&mut self, x: i32, y: i32) {
        Rectangle::new(Point::new(self.cursor_x, self.cursor_y), Size::new(7, 9))
            .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
            .draw(self.display)
            .unwrap();
        let x_offset = x - self.cursor_x;
        let y_offset = y - self.cursor_y;
        self.cursor_x = x;
        self.cursor_y = y;

        self.cursor_shape.translate_mut(Point::new(x_offset, y_offset));
        self.cursor_shape.draw(self.display).unwrap()
    }

    pub fn set_focus(&mut self, start: Link<F>) {
        self.current = start.clone();
        if let Some(start) = start {
            if let Some(draw) = start.borrow().draw {
                draw(self.display);
            }
        }
    }

    pub fn iter_horizontal(&self) -> HorizontalIterator<F> {
        HorizontalIterator { current: self.head.clone() }
    }

    pub fn iter_vertical(&self) -> VerticalIterator<F> {
        VerticalIterator { current: self.head.clone() }
    }
}

struct VerticalIterator<F> {
    current: Link<F>,
}

impl<F> Iterator for VerticalIterator<F> {
    type Item = Rc<RefCell<Level<F>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_node = self.current.clone();
        self.current = next_node.as_ref().and_then(|node| node.borrow().down.clone());
        next_node
    }
}

struct HorizontalIterator<F> {
    current: Link<F>,
}

impl<F> Iterator for HorizontalIterator<F> {
    type Item = Rc<RefCell<Level<F>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_node = self.current.clone();
        self.current = next_node.as_ref().and_then(|node| node.borrow().next.clone());
        next_node
    }
}

pub struct Level<F> {
    draw: Option<fn(&mut ILI9488)>,
    next: Link<F>,
    prev: Link<F>,
    up: Link<F>,
    down: Link<F>,
    x: i32,
    y: i32
}

impl<F> Level<F> {
    pub fn new(draw: Option<fn(&mut ILI9488)>, x: i32, y: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Level {
            draw,
            next: None,
            prev: None,
            up: None,
            down: None,
            x,
            y
        }))
    }
}

pub fn show_loadscreen_logo(display: &mut ILI9488) {
    let bmp: Bmp<Rgb888> = Bmp::from_slice(include_bytes!("../assets/loadscreen_logo.bmp")).unwrap();
    let image = Image::new(&bmp, Point::new(0, 0));

    let background = Rectangle::new(Point::new(0, 0), Size::new(320, 480))
        .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK));

    image.draw(display).expect("Expected to draw loadscreen logo");
    background.draw(display).expect("Expected to draw background");
}

pub fn show_main_menu(display: &mut ILI9488) {
    let top_bar_rectangle = Rectangle::new(Point::new(0, 0), Size::new(320, 20))
        .into_styled(PrimitiveStyle::with_fill(Rgb888::YELLOW));

    let border_rectangle = Rectangle::new(Point::new(0, 20), Size::new(320, 460))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::YELLOW, 2));

    let top_bar_text_style = MonoTextStyle::new(&FONT_7X14, Rgb888::BLACK);
    let top_bar_text = Text::new("CHEETAH v0.1", Point::new(5, 14), top_bar_text_style);

    top_bar_rectangle.draw(display).expect("Expected to draw top rectangle");
    border_rectangle.draw(display).expect("Expected to draw border rectangle");
    top_bar_text.draw(display).expect("Expected to draw top bar text");

    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb888::YELLOW);

    let text_wifi = Text::new("WIFI", Point::new(25, 45), text_style);
    let text_bluetooth = Text::new("BLUETOOTH", Point::new(25, 70), text_style);
    let text_settings = Text::new("SETTINGS", Point::new(25, 95), text_style);

    text_wifi.draw(display).expect("Unable to draw text");
    text_bluetooth.draw(display).expect("Unable to draw text");
    text_settings.draw(display).expect("Unable to draw text");
}

pub fn show_wifi_menu(display: &mut ILI9488) {
    for y in (10..=60).step_by(25) {
        let filled_rectangle = Rectangle::new(Point::new(12, y), Size::new(300, 20))
            .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK)); // Specify the fill color here

        // Draw the rectangle on the display
        filled_rectangle.draw(display).expect("Unable to draw rectangle");
    }

    // Define the text style
    let text_style = MonoTextStyle::new(&FONT_9X15, Rgb888::YELLOW);

    // Create a text object
    let text_wifi_scanner = Text::new("WIFI SCANNER", Point::new(25, 45), text_style);
    let text_wifi_sniffer = Text::new("WIFI SNIFFER", Point::new(25, 70), text_style);
    let text_wifi_deauther = Text::new("WIFI DEAUTHER", Point::new(25, 95), text_style);

    // Draw the text on the display
    text_wifi_scanner.draw(display).expect("Unable to draw text");
    text_wifi_sniffer.draw(display).expect("Unable to draw text");
    text_wifi_deauther.draw(display).expect("Unable to draw text");
}

// pub fn hide_wifi_menu(display: &mut ILI9488) {
//
// }
//
// pub fn show_bluetooth_menu(display: &mut ILI9488) {
//
// }
//
// pub fn hide_bluetooth_menu(display: &mut ILI9488) {
//
// }
//
// pub fn show_settings_menu(display: &mut ILI9488) {
//
// }
//
// pub fn hide_settings_menu(display: &mut ILI9488) {
//
// }