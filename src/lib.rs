#[cfg(test)]
mod tests {
    use sfml::{
        graphics::{Color, RenderWindow, RenderTarget},
        system::{Vector2f},
        window::{Style, Event, Key},
    };

    use crate::menu::Menu;

    #[test]
    fn run_system() {
        let mut window = RenderWindow::new(
            (1920, 1080),
            "SFML window",
            Style::CLOSE | Style::FULLSCREEN,
            &Default::default()
        );

        let mut my_menu: Menu = Menu::with_position(Vector2f::new(64.0, 128.0));
        my_menu.add_option(String::from("Play."), String::from("RedHatMono-Regular.ttf"), Vector2f::new(0.0, 0.0), 32, Color::GREEN);
        my_menu.add_option(String::from("Settings."), String::from("RedHatMono-Regular.ttf"), Vector2f::new(0.0, 0.0), 32, Color::GREEN);
        my_menu.add_option(String::from("Exit."), String::from("RedHatMono-Regular.ttf"), Vector2f::new(0.0, 0.0), 32, Color::GREEN);

        while window.is_open() {

            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                    _ => {}
                }

                my_menu.event(&event);
            }

            window.clear(Color::BLACK);
            my_menu.draw(&mut window);
            window.display();
        }
    }
}

pub mod menu {
    use sfml:: {
        graphics::{Color, Font, RenderWindow, Text, TextStyle, RenderTarget, Transformable},
        system::{Vector2f},
        window::{Event, Key},
    };

    use crate::menu::menu_text::MenuText;

    pub struct Menu {
        options: Vec<MenuText>,
        curr_option: u32,

        position: Vector2f,
    }

    impl Menu {
        pub fn new() -> Self {
            Self {
                options: Vec::new(),
                curr_option: 0,

                position: Vector2f::new(0.0, 0.0),
            }
        }

        pub fn with_position(position: Vector2f) -> Self {
            Self {
                options: Vec::new(),
                curr_option: 0,

                position: position,
            }
        }

        pub fn add_option_auto(&mut self) {
            self.options.push(MenuText::new_auto());
        }

        pub fn add_option(&mut self, text: String, font: String, position: Vector2f, size: u32, color: Color) {
            if self.options.len() == 0 {
                self.options.push(MenuText::new(text, font, position, size, color));
            } else {
                let last = (self.options.len() as u32 - 1) as usize;
                self.options.push(MenuText::new(text, font, Vector2f::new(self.options[last].get_position().x, ((self.options[last].get_size() + 32) * (last as u32 + 1)) as f32), size, color));
            }
        }

        pub fn event(&mut self, event: &Event) {
            match event {
                Event::KeyPressed { code: Key::Down, .. } => {
                    if self.curr_option < self.options.len() as u32 - 1
                    { self.curr_option += 1; }
                    else
                    { self.curr_option = 0; }
                }
                Event::KeyPressed { code: Key::Up, .. } => {
                    if self.curr_option > 0
                    { self.curr_option -= 1; }
                    else
                    { self.curr_option = self.options.len() as u32 - 1; }
                }

                _ => {}
            }
        }

        pub fn draw(&mut self, window: &mut RenderWindow) {
            for i in 0..self.options.len() {
                let tmp_font = Font::from_file(self.options[i].get_font()).unwrap();
                let mut tmp_text = Text::new(self.options[i].get_text(), &tmp_font, *self.options[i].get_size());

                tmp_text.set_position(Vector2f::new(self.options[i].get_position().x + self.position.x, self.options[i].get_position().y + self.position.y));
                tmp_text.set_character_size(*self.options[i].get_size());

                tmp_text.set_fill_color(*self.options[i].get_color());

                if i as u32 == self.curr_option {
                    tmp_text.set_style(TextStyle::UNDERLINED);
                } else {
                    tmp_text.set_style(TextStyle::REGULAR);
                }

                window.draw(&tmp_text);
            }
        }
    }

    mod menu_text {
        use crate::menu::*;

        pub struct MenuText {
            text: String,
            font: String,

            position: Vector2f,
            size: u32,

            color: Color,
        }

        impl MenuText {
            pub fn new_auto() -> Self {
                Self {
                    text: String::from("Blank"),
                    font: String::from("RedHatMono-Regular.ttf"),

                    position: Vector2f::new(0.0, 0.0),
                    size: 32,

                    color: Color::GREEN,
                }
            }

            pub fn new(text: String, font: String, position: Vector2f, size: u32, color: Color) -> Self {
                Self {
                    text,
                    font,

                    position,
                    size,

                    color,
                }
            }

            pub fn get_text(&self) -> &String { &self.text }
            pub fn get_font(&self) -> &String { &self.font }

            pub fn get_position(&self) -> &Vector2f { &self.position }
            pub fn get_size(&self) -> &u32 { &self.size }

            pub fn get_color(&self) -> &Color { &self.color }
        }
    }
}
