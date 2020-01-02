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

external libscroll_push_pan: (scrollview, axis, float) => unit = "libscroll_push_pan";
external libscroll_push_interrupt: scrollview => unit = "libscroll_push_interrupt";
external libscroll_push_fling: scrollview => unit = "libscroll_push_fling";
external libscroll_get_position_absolute: scrollview => (float, float) = "libscroll_get_position_absolute";
external libscroll_set_next_frame_predict: (scrollview, float) => unit = "libscroll_set_next_frame_predict";
external libscroll_set_avg_frametime: (scrollview, float) => unit = "libscroll_set_avg_frametime";
external libscroll_step_frame: scrollview => unit = "libscroll_step_frame";
external libscroll_animating: scrollview => bool = "libscroll_animating";
external libscroll_set_geometry: (scrollview, float, float, float, float) => unit = "libscroll_set_geometry";
external libscroll_new: unit => scrollview = "libscroll_new";
external libscroll_del: scrollview => unit = "libscroll_del";
external libscroll_set_source: (scrollview, source) => unit = "libscroll_set_source";
