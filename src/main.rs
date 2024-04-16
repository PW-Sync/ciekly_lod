// hello world

slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        VerticalBox {
            Text { text: "hello world."; }
            Button { text: "wow"; }
        }
    }
}

fn main() {
    App::new().unwrap().run().unwrap();
    println!("hello world");
}
