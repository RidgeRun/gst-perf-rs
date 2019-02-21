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

use gst;
use gobject_subclass::object::*;
use gst_plugin::base_transform::*;
use gst_plugin::element::*;

struct Perf {
    cat: gst::DebugCategory,
}

impl Perf {
    fn new(_transform: &BaseTransform) -> Box<BaseTransformImpl<BaseTransform>> {
        Box::new(Self {
            cat: gst::DebugCategory::new(
                "rsperf",
                gst::DebugColorFlags::empty(),
                "Rust RidgeRun Perf element",
            ),
        })
    }
    fn class_init(klass: &mut BaseTransformClass) {
        klass.set_metadata(
            "Performance Identity element",
            "Generic",
            "Get pipeline performance data",
            "Carlos Rodriguez <carlos.rodriguez@ridgerun.com>"
        );

        let caps = gst::Caps::new_any();
        
        let src_pad_template = gst::PadTemplate::new(
            "src",
            gst::PadDirection::Src,
            gst::PadPresence::Always,
            &caps,
        );
        klass.add_pad_template(src_pad_template);
        
        let caps = gst::Caps::new_any();
        let sink_pad_template = gst::PadTemplate::new(
            "sink",
            gst::PadDirection::Sink,
            gst::PadPresence::Always,
            &caps,
        );
        klass.add_pad_template(sink_pad_template);

        klass.configure(BaseTransformMode::AlwaysInPlace, false, false);
    }
}

impl ObjectImpl<BaseTransform> for Perf {}
impl ElementImpl<BaseTransform> for Perf {}

impl BaseTransformImpl<BaseTransform> for Perf {
    fn transform_ip(&self, _element: &BaseTransform, _buf: &mut gst::BufferRef)-> gst::FlowReturn {
        let time = gst::util_get_timestamp();

        gst_info!(self.cat, obj: _element, "Current Timestamp is: {}", time);

        return gst::FlowReturn::Ok
    }
}

struct PerfStatic;

impl ImplTypeStatic<BaseTransform> for PerfStatic {
    fn get_name(&self) -> &str {
        "Perf"
    }
    fn new(&self, element: &BaseTransform) -> Box<BaseTransformImpl<BaseTransform>> {
        Perf::new(element)
    }
    fn class_init(&self, klass: &mut BaseTransformClass) {
        Perf::class_init(klass);
    }
}

pub fn register(plugin: &gst::Plugin) {
    let type_ = register_type(PerfStatic);
    gst::Element::register(plugin, "rsperf", 0, type_);
}
