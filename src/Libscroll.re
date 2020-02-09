type scrollview;
type axis = 
  | Horizontal
  | Vertical;

type source =
  | Undefined
  | Touchscreen
  | Touchpad
  | Mousewheel
  | PreciseMousewheel
  | Passthrough
  | KineticPassthrough
  | Previous;

external push_pan: (scrollview, axis, float) => unit = "libscroll_push_pan";
external push_interrupt: scrollview => unit = "libscroll_push_interrupt";
external push_fling: scrollview => unit = "libscroll_push_fling";
external get_position_absolute: scrollview => (float, float) = "libscroll_get_position_absolute";
external set_next_frame_predict: (scrollview, float) => unit = "libscroll_set_next_frame_predict";
external set_avg_frametime: (scrollview, float) => unit = "libscroll_set_avg_frametime";
external step_frame: scrollview => unit = "libscroll_step_frame";
external animating: scrollview => bool = "libscroll_animating";
external set_geometry: (scrollview, float, float, float, float) => unit = "libscroll_set_geometry";
external scrollview_new: unit => scrollview = "libscroll_new";
external scrollview_del: scrollview => unit = "libscroll_del";
external set_source: (scrollview, source) => unit = "libscroll_set_source";

external libscroll_sanity: unit => bool = "libscroll_sanity";

let libscroll_sanity_internal = () => Printf.eprintf("hello from libscroll internal");
