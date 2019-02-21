// Copyright (C) 2019 Carlos Rodriguez <carlos.rodriguez@ridgerun.com>
//
// LICENSE

extern crate gobject_subclass;
#[macro_use]
extern crate gst_plugin;
#[macro_use]
extern crate gstreamer as gst;

mod perf;

fn plugin_init(plugin: &gst::Plugin) -> bool {
    perf::register(plugin);
    true
}

plugin_define!(
    b"rsperf\0",
    b"Rust RidgeRun Perf Element\0",
    plugin_init,
    b"1.0\0",
    b"LGPL\0",
    b"rsperf\0",
    b"rsperf\0",
    b"https://github.com/RidgeRun/gst-perf-rs\0",
    b"2019-02-20\0"
);
