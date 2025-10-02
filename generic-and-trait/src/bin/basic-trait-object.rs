pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // create `run` method.
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let select_box = SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("No"),
            String::from("Maybe"),
        ],
    };

    let button = Button {
        width: 50,
        height: 10,
        label: String::from("Ok"),
    };

    let screen = Screen {
        components: vec![Box::new(select_box), Box::new(button)],
    };

    screen.run()
}
