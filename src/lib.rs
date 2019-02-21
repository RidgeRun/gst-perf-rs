/*
 * GStreamer
 * Copyright (C) 2019 RidgeRun
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 *
 */

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
