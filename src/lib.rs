extern crate libscroll;
extern crate num;



#[macro_use]
extern crate ocaml;

static BAD_PTR: &str = "ERROR: null/invalid ptr passed as scrollview handle";

//use ocaml::core::memory;
use ocaml::{ToValue, Value};

caml!(libscroll_new() {
    //std::mem::transmute::<*mut libscroll::Scrollview, Value>(Box::into_raw(Box::new(libscroll::Scrollview::new())))

    caml_local!(result);

    //result = std::mem::transmute(Box::into_raw(Box::new(libscroll::Scrollview::new())));

    //return result;
    Value::ptr(Box::into_raw(Box::new(libscroll::Scrollview::new())))

});

caml!(libscroll_del(scrollview) {
    let _: Box<libscroll::Scrollview> = Box::from_raw(std::mem::transmute(scrollview));
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
    let content_height = content_height.int64_val() as u64;
    let content_width = content_width.int64_val() as u64;
    let viewport_width = viewport_width.int64_val() as u64;
    let viewport_height = viewport_height.int64_val() as u64;

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

    let result: isize = scrollview
        .as_mut()
        .expect(BAD_PTR)
        .animating()
        .into();

    //return scrollview.animating().bool_val()
    Value::nativeint(result)
});

caml!(libscroll_step_frame(scrollview) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .step_frame(None);


    Value::unit()
});

caml!(libscroll_set_avg_frametime(scrollview, milliseconds) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let milliseconds = milliseconds.f64_val();

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .set_avg_frametime(milliseconds);

    Value::unit()
});

caml!(libscroll_set_next_frame_predict(scrollview, milliseconds) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let milliseconds = milliseconds.f64_val();

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .set_next_frame_predict(milliseconds);

    Value::unit()
});

caml!(libscroll_get_position_absolute(scrollview) {
    let scrollview = scrollview.ptr_val::<libscroll::Scrollview>();

    let pos = scrollview
        .as_ref()
        .expect(BAD_PTR)
        .get_position_absolute();

    tuple!(pos.x, pos.y).into()
});

caml!(libscroll_push_pan(scrollview, axis, amount) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    let axis = axis.nativeint_val();
    let axis = match axis {
        0 => libscroll::Axis::Horizontal,
        1 => libscroll::Axis::Vertical,
        other => panic!("Bad input for axis bounds {}, expected 0 (horizontal) or 1 (vertical)", other),
    };

    let amount = amount.f64_val();

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .push_pan(None, axis, amount);

    Value::unit()
});

caml!(libscroll_push_fling(scrollview) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .push_fling();

    Value::unit()
});

caml!(libscroll_push_interrupt(scrollview) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .push_interrupt();

    Value::unit()
});

caml!(libscroll_set_source(scrollview, source) {
    let scrollview = scrollview.mut_ptr_val::<libscroll::Scrollview>();
    //let source: libscroll::Source = std::mem::transmute(source.nativeint_val());
    //let source: Option<libscroll::Source> = num::FromPrimitive::from_isize(source.nativeint_val());
    let source: libscroll::Source = std::mem::transmute(source.usize_val() as u8);

    scrollview
        .as_mut()
        .expect(BAD_PTR)
        .set_source(source);

    Value::unit()
});
