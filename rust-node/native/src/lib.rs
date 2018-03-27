#[macro_use]
extern crate neon;
extern crate markdown;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use neon::js::JsInteger;//jsString es para String y JsInteger es para jalar ints
use neon::mem::Handle;

fn render(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let md: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
    let string = md.value();
    let html: String = markdown::to_html(&string);

    Ok(JsString::new(scope, &html).unwrap())
}

fn fibonacci(n: i32) -> i32 {
    return match n {
        1 | 2 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn method(call: Call) -> JsResult<JsInteger> {
      let scope = call.scope;
      let x = try!(try!(call.arguments.require(scope, 0)).check::<JsInteger>()).value();

      Ok(JsInteger::new(scope, fibonacci(x as i32)))
}

register_module!(m, {
    m.export("fibonacci", method)
    // m.export("render",render)
});
