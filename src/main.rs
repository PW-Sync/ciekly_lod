/*slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        VerticalBox {
            Text { text: "hello world."; }
            Button { text: "wow"; }
        }
    }
}*/

slint::include_modules!();
fn main() {
    App::new().unwrap().run().unwrap();
    println!("hello world");
}
