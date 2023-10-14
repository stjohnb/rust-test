// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
  // An `enum` variant may either be `unit-like`,
  PageLoad,
  PageUnload,
  // like tuple structs,
  KeyPress(char),
  Paste(String),
  Test(String),
  // or c-like structures.
  Click { x: i64, y: i64 },
}

enum WebEvent2 {
  PageLoad,
  PageUnload
}


// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
  match event {
      WebEvent::PageLoad => println!("page loaded"),
      WebEvent::PageUnload => println!("page unloaded"),
      // Destructure `c` from inside the `enum` variant.
      WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
      WebEvent::Paste(s) => println!("pasted \"{}\".", s),
      WebEvent::Test(s) => println!("test \"{}\".", s),
      // Destructure `Click` into `x` and `y`.
      WebEvent::Click { x, y } => {
          println!("clicked at x={}, y={}.", x, y);
      },
  }
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted  = WebEvent::Paste("my text".to_owned());
  let tested  = WebEvent::Test("my test".to_owned());
  let click   = WebEvent::Click { x: 20, y: 80 };
  let load    = WebEvent::PageLoad;
  let unload  = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(tested);
  inspect(click);
  inspect(load);
  inspect(unload);

  println!("WebEvent2::PageLoad is #{:06x}", WebEvent2::PageLoad as i32);
  println!("WebEvent2::PageUnload is #{:06x}", WebEvent2::PageUnload as i32);
}
