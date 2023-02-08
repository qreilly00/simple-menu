#[cfg(test)]
mod tests {
    use crate::menu::Menu;

    use sfml::{
        graphics::{Color, RenderWindow, RenderTarget},
        system::{Vector2f},
        window::{Event, Key, Style},
    };

    #[test]
    fn test_menu() {
        let mut window = RenderWindow::new(
            (1280, 720),
            "SFML window",
            Style::CLOSE,
            &Default::default()
        );

        let mut my_menu: Menu = Menu::new();
        my_menu.add_text(String::from("Test 1."), String::from("RedHatMono.ttf"), 32, Vector2f::new(16.0, 0.0), Color::WHITE);
        my_menu.add_text(String::from("Test 2."), String::from("RedHatMono.ttf"), 32, Vector2f::new(16.0, 64.0), Color::WHITE);
        my_menu.add_text(String::from("Test 3."), String::from("RedHatMono.ttf"), 32, Vector2f::new(16.0, 128.0), Color::WHITE);

        while window.is_open() {

            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),

                    _ => {}
                }

                my_menu.event(event);
            }

            window.clear(Color::BLACK);
            my_menu.draw(&mut window);
            window.display();
        }
    }

}

mod menu {
    use sfml::{
        graphics::{Font, Text, TextStyle, Color, RenderWindow, RenderTarget, Transformable},
        system::{Vector2f},
        window::{Event, Key},
    };

    use crate::menu::menu_text::*;

    pub struct Menu {
        options: Vec<MenuText>,
        curr_option: u32,
    }

    impl Menu {
        pub fn new() -> Self{
            Self{
                options: Vec::new(),
                curr_option: 0,
            }
        }

        pub fn add_text(&mut self, caption: String, font: String, size: u32, position: Vector2f, text_color: Color) {
            self.options.push(MenuText::with_params(caption, font, size, position, text_color));
        }

        pub fn draw(&self, window: &mut RenderWindow) {
            for i in 0..self.options.len() {
                self.options[i].draw(window, self.curr_option == i as u32);
            }
        }

        pub fn event(&mut self, event: Event) {
            match event {
                Event::KeyPressed { code: Key::Down, .. } => {
                    if self.curr_option < self.options.len() as u32 - 1 {
                        self.curr_option += 1;
                    } else {
                        self.curr_option = 0;
                    }
                }
                Event::KeyPressed { code: Key::Up, .. } => {
                    if self.curr_option > 0 {
                        self.curr_option -= 1;
                    } else {
                        self.curr_option = self.options.len() as u32 - 1;
                    }
                }

                _ => {}
            }
        }
    }

    mod menu_text {
        use crate::menu::*;

        pub struct MenuText {
            caption: String,
            font: String,
            size: u32,

            position: Vector2f,
            text_color: Color,
        }

        impl MenuText {
            pub fn new() -> Self {
                Self {
                    caption: String::from("Test."),
                    font: String::from("RedHatMono.ttf"),
                    size: 30,

                    position: Vector2f::new(0.0, 0.0),
                    text_color: Color::WHITE,
                }
            }

            pub fn with_params(caption: String, font: String, size: u32, position: Vector2f, text_color: Color) -> Self {
                Self {
                    caption,
                    font,
                    size,

                    position,
                    text_color,
                }
            }

            pub fn draw(&self, window: &mut RenderWindow, underline: bool) {
                let tmp_font = Font::from_file(self.get_font()).unwrap();
                let mut tmp_text = Text::new(self.get_caption(), &tmp_font, *self.get_size());

                tmp_text.set_position(*self.get_position());
                tmp_text.set_fill_color(*self.get_text_color());

                if underline == true {
                    tmp_text.set_style(TextStyle::UNDERLINED);
                } else {
                    tmp_text.set_style(TextStyle::REGULAR);
                }

                window.draw(&tmp_text);
            }

            pub fn get_caption(&self) -> &String { &self.caption }
            pub fn get_font(&self) -> &String { &self.font }
            pub fn get_size(&self) -> &u32 { &self.size }

            pub fn get_position(&self) -> &Vector2f { &self.position }
            pub fn get_text_color(&self) -> &Color { &self.text_color }
        }
    }
}
