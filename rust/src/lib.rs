extern crate libscroll;
extern crate num;



#[macro_use]
extern crate ocaml;

static BAD_PTR: &str = "ERROR: null/invalid ptr passed as scrollview handle";

//use ocaml::core::memory;
use ocaml::{ToValue, Value};
use ocaml::core::memory;

//static mut VIEWS: Option<std::sync::Mutex<std::collections::HashMap<i64, libscroll::Scrollview>>> = None;

/*caml!(libscroll_get_by_handle(handle) {
    //
});*/

caml!(libscroll_new() {
    //std::mem::transmute::<*mut libscroll::Scrollview, Value>(Box::into_raw(Box::new(libscroll::Scrollview::new())))
    //VIEWS = Some(std::sync::Mutex::new(std::collections::HashMap::new()));

    caml_local!(result);

    //result = std::mem::transmute(Box::into_raw(Box::new(libscroll::Scrollview::new())));
    println!("Libscroll: Creating new scrollview!");

    //return result;
    Value::ptr(Box::into_raw(Box::new(libscroll::Scrollview::new())))

});

caml!(libscroll_del(scrollview) {
    let _: Box<libscroll::Scrollview> = Box::from_raw(std::mem::transmute(scrollview));

    println!("Libscroll: Dropped scrollview");
    // drops

    Value::unit()
});

caml!(libscroll_set_geometry(
        scrollview,
        content_height,
        content_width,
        viewport_height,
        viewport_width
) {
    //let mut scrollview: Box<libscroll::Scrollview> = Box::from_raw(std::mem::transmute(scrollview));
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let content_height = content_height.f64_val();
    let content_width = content_width.f64_val();
    let viewport_width = viewport_width.f64_val();
    let viewport_height = viewport_height.f64_val();

    //println!("Libscroll: Setting scrollview geometry");

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .set_geometry(
            content_height,
            content_width,
            viewport_height,
            viewport_width,
        );

    Value::unit()
});

caml!(libscroll_animating(scrollview) {
    //let scrollview: Box<libscroll::Scrollview> = Box::from_raw(std::mem::transmute(scrollview));
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();

    let result: bool = scrollview
        .as_mut()
        .expect(BAD_PTR)
        .animating();

    //println!("Libscroll: animating returns: {}", result);

    //return scrollview.animating().bool_val()
    //scrollview.animating().bool_val();
    //scrollview.bool_val();
    Value::bool(result)
    //Value::nativeint(result)
});

/*caml!(libscroll_step_frame(scrollview) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();

    println!("Libscroll: stepping frame");

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .step_frame(None);


    Value::unit()
});*/

/*caml!(libscroll_set_avg_frametime(scrollview, milliseconds) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let milliseconds = milliseconds.f64_val();

    println!("Libscroll: Setting avg frametime to {} milliseconds", milliseconds);

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .set_avg_frametime(milliseconds);

    Value::unit()
});*/

/*caml!(libscroll_set_next_frame_predict(scrollview, milliseconds) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let milliseconds = milliseconds.f64_val();

    println!("Libscroll: predict next frame in {} millis", milliseconds);

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .set_next_frame_predict(milliseconds);

    Value::unit()
});*/

/*caml!(libscroll_get_position_absolute(scrollview) {
    let scrollview = scrollview.ptr_val::<libscroll::Scrollview>();

    println!("Libscroll: Was requested position absolute");

    let pos = scrollview
        .as_ref()
        .expect(BAD_PTR)
        .get_position_absolute();

    tuple!(pos.x, pos.y).into()
});*/

caml!(libscroll_sample(scrollview, timestamp_micros) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let timestamp_micros = timestamp_micros.i64_val() as u64;

    //println!("Sampling for libscroll! Timestamp: {}", timestamp_micros);

    let result = scrollview
        .as_mut()
        .expect(BAD_PTR)
        .sample(timestamp_micros);

    //println!("\tGets {}, {}", result.x, result.y);

    tuple!(result.x, result.y).into()
});

caml!(libscroll_push_pan(scrollview, axis, amount, timestamp) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let axis = axis.usize_val();
    let axis = match axis {
        0 => libscroll::Axis::Horizontal,
        1 => libscroll::Axis::Vertical,
        other => panic!("Bad input for axis bounds {}, expected 0 (horizontal) or 1 (vertical)", other),
    };
    let timestamp = timestamp.i64_val() as u64;

    let amount = amount.f64_val();

    //println!("Libscroll: pushing pan on axis {:?} with amount {}", axis, amount);
    //println!("Libscroll: pushing pan of amount {}", amount);

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .push_pan(axis, amount, Some(timestamp));

    Value::unit()
});

caml!(libscroll_push_fling(scrollview, axis, timestamp) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let timestamp = timestamp.i64_val() as u64;
    //println!("Unwraps fling timestamp to {}", timestamp);

    //println!("Libscroll: pushing fling");

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .push_fling(Some(timestamp));

    Value::unit()
});

caml!(libscroll_push_interrupt(scrollview, timestamp) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let timestamp = timestamp.i64_val() as u64;

    //println!("Libscroll: pushing interrupt");

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .push_interrupt(Some(timestamp));

    Value::unit()
});

caml!(libscroll_set_source(scrollview, source) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    //let source: libscroll::Source = std::mem::transmute(source.nativeint_val());
    //let source: Option<libscroll::Source> = num::FromPrimitive::from_isize(source.nativeint_val());
    let source: libscroll::Source = std::mem::transmute(source.usize_val() as u8);

    //println!("Libscroll: setting source to {:?}", source);
    //println!("Libscroll: setting source: {:?}", source);

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .set_source(source);

    Value::unit()
});

caml!(libscroll_sanity() {
    println!("Libscroll is reachable!");

    Value::unit()
});

/*#[no_mangle]
pub extern fn libscroll_sanity() -> Value {
    println!("Libscroll is reachable!");

    Value::unit()
}*/
