use gtk::{
  glib::{self},
  prelude::*,
  ApplicationWindow, EventBox, HeaderBar,
};

pub struct WlHeader;

impl WlHeader {
  pub fn setup(window: &ApplicationWindow, title: &str, min_width: i32) {
    let header = HeaderBar::builder()
      .show_close_button(true)
      .decoration_layout("menu:minimize,maximize,close")
      .title(title)
      .build();

    let event_box = EventBox::new();
    event_box.set_above_child(true);
    event_box.set_visible(true);
    event_box.set_can_focus(false);

    let header_clone = header.clone();
    let event_box_clone = event_box.clone();
    glib::idle_add_local(move || {
      let allocated_height = header_clone.allocated_height();
      event_box_clone.set_size_request(min_width, allocated_height);
      header_clone.set_size_request(min_width, allocated_height);
      glib::ControlFlow::Break
    });

    event_box.add(&header);
    window.set_titlebar(Some(&event_box));
    Self::connect_resize_window(&header, window);
  }

  fn connect_resize_window(header: &HeaderBar, window: &ApplicationWindow) {
    let header_weak = header.downgrade();
    window.connect_resizable_notify(move |window| {
      if let Some(header) = header_weak.upgrade() {
        let is_resizable = window.is_resizable();
        header.set_decoration_layout(if !is_resizable {
          Some("menu:minimize,close")
        } else {
          Some("menu:minimize,maximize,close")
        });
      }
    });
  }
}
