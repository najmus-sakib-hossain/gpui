Please do a web search and fix these errors and warnings:

error[E0422]: cannot find struct, variant or union type `Frame` in this scope
  --> src\bitmap_utils.rs:21:17
   |
21 |     let frame = Frame {
   |                 ^^^^^ not found in this scope
   |
help: consider importing one of these items
   |
 6 + use crate::IconName::Frame;
   |
 6 + use gpui_component::IconName::Frame;
   |
 6 + use image::Frame;
   |
 6 + use typst::layout::Frame;
   |
   = and 2 other candidates

error[E0433]: failed to resolve: could not find `Document` in `hayro`
   --> src\pdf_view.rs:171:27
    |
171 |     let document = hayro::Document::parse(&pdf_data)?;
    |                           ^^^^^^^^ could not find `Document` in `hayro`
    |
help: consider importing one of these items
    |
  6 + use docx_rs::Document;
    |
  6 + use typst::Document;
    |
help: if you import `Document`, refer to it directly
    |
171 -     let document = hayro::Document::parse(&pdf_data)?;
171 +     let document = Document::parse(&pdf_data)?;
    |

warning: unused import: `symphonia::core::codecs::DecoderOptions`
  --> src\video_view.rs:12:5
   |
12 | use symphonia::core::codecs::DecoderOptions;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `typst::foundations::Smart`
   --> src\latex_view.rs:222:9
    |
222 |     use typst::foundations::Smart;
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
   --> src\video_view.rs:102:14
    |
101 | /         div()
102 | |             .v_flex()
    | |_____________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
    |
 73 |       fn v_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `h_flex` with a similar name
    |
102 -             .v_flex()
102 +             .h_flex()
    |

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
   --> src\video_view.rs:144:22
    |
143 | /                 div()
144 | |                     .h_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
    |
 67 |       fn h_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `v_flex` with a similar name
    |
144 -                     .h_flex()
144 +                     .v_flex()
    |

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\video_view.rs:157:36
     |
 157 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 157 |                                 cx.notify(/* EntityId */);
     |                                           ++++++++++++++

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
  --> src\three_d_view.rs:81:14
   |
80 | /         div()
81 | |             .v_flex()
   | |_____________-^^^^^^
   |
  ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
   |
73 |       fn v_flex(self) -> Self {
   |          ------ the method is available for `gpui::Div` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
   |
 6 + use gpui_component::StyledExt;
   |
help: there is a method `h_flex` with a similar name
   |
81 -             .v_flex()
81 +             .h_flex()
   |

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
   --> src\three_d_view.rs:118:22
    |
117 | /                 div()
118 | |                     .h_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
    |
 67 |       fn h_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `v_flex` with a similar name
    |
118 -                     .h_flex()
118 +                     .v_flex()
    |

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\three_d_view.rs:130:36
     |
 130 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 130 |                                 cx.notify(/* EntityId */);
     |                                           ++++++++++++++

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\three_d_view.rs:143:36
     |
 143 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 143 |                                 cx.notify(/* EntityId */);
     |                                           ++++++++++++++

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
   --> src\audio_view.rs:104:14
    |
103 | /         div()
104 | |             .v_flex()
    | |_____________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
    |
 73 |       fn v_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `h_flex` with a similar name
    |
104 -             .v_flex()
104 +             .h_flex()
    |

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
   --> src\audio_view.rs:120:22
    |
119 | /                 div()
120 | |                     .v_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
    |
 73 |       fn v_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `h_flex` with a similar name
    |
120 -                     .v_flex()
120 +                     .h_flex()
    |

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
   --> src\audio_view.rs:137:22
    |
136 | /                 div()
137 | |                     .h_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
    |
 67 |       fn h_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `v_flex` with a similar name
    |
137 -                     .h_flex()
137 +                     .v_flex()
    |

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\audio_view.rs:154:36
     |
 154 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 154 |                                 cx.notify(/* EntityId */);
     |                                           ++++++++++++++

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\audio_view.rs:167:36
     |
 167 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 167 |                                 cx.notify(/* EntityId */);
     |                                           ++++++++++++++

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
   --> src\audio_view.rs:174:22
    |
173 | /                 div()
174 | |                     .h_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
    |
 67 |       fn h_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `v_flex` with a similar name
    |
174 -                     .h_flex()
174 +                     .v_flex()
    |

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
  --> src\pdf_view.rs:77:14
   |
76 | /         div()
77 | |             .v_flex()
   | |_____________-^^^^^^
   |
  ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
   |
73 |       fn v_flex(self) -> Self {
   |          ------ the method is available for `gpui::Div` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
   |
 6 + use gpui_component::StyledExt;
   |
help: there is a method `h_flex` with a similar name
   |
77 -             .v_flex()
77 +             .h_flex()
   |

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
   --> src\pdf_view.rs:119:22
    |
118 | /                 div()
119 | |                     .h_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
    |
 67 |       fn h_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
    |
  6 + use gpui_component::StyledExt;
    |
help: there is a method `v_flex` with a similar name
    |
119 -                     .h_flex()
119 +                     .v_flex()
    |

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\pdf_view.rs:132:36
     |
 132 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 132 |                                 cx.notify(/* EntityId */);
     |                                           ++++++++++++++

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\pdf_view.rs:154:36
     |
 154 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 154 |                                 cx.notify(/* EntityId */);
     |                                           ++++++++++++++

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
  --> src\doc_view.rs:46:14
   |
45 | /         div()
46 | |             .v_flex()
   | |_____________-^^^^^^
   |
  ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
   |
73 |       fn v_flex(self) -> Self {
   |          ------ the method is available for `gpui::Div` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
   |
 6 + use gpui_component::StyledExt;
   |
help: there is a method `h_flex` with a similar name
   |
46 -             .v_flex()
46 +             .h_flex()
   |

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
  --> src\doc_view.rs:59:22
   |
58 | /                 div()
59 | |                     .v_flex()
   | |_____________________-^^^^^^
   |
  ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
   |
73 |       fn v_flex(self) -> Self {
   |          ------ the method is available for `gpui::Div` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
   |
 6 + use gpui_component::StyledExt;
   |
help: there is a method `h_flex` with a similar name
   |
59 -                     .v_flex()
59 +                     .h_flex()
   |

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
  --> src\latex_view.rs:99:14
   |
98 | /         div()
99 | |             .v_flex()
   | |_____________-^^^^^^
   |
  ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
   |
73 |       fn v_flex(self) -> Self {
   |          ------ the method is available for `gpui::Div` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
   |
 7 + use gpui_component::StyledExt;
   |
help: there is a method `h_flex` with a similar name
   |
99 -             .v_flex()
99 +             .h_flex()
   |

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
   --> src\latex_view.rs:105:22
    |
104 | /                 div()
105 | |                     .h_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
    |
 67 |       fn h_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
    |
  7 + use gpui_component::StyledExt;
    |
help: there is a method `v_flex` with a similar name
    |
105 -                     .h_flex()
105 +                     .v_flex()
    |

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
   --> src\latex_view.rs:147:22
    |
146 | /                 div()
147 | |                     .h_flex()
    | |_____________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
    |
 67 |       fn h_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
    |
  7 + use gpui_component::StyledExt;
    |
help: there is a method `v_flex` with a similar name
    |
147 -                     .h_flex()
147 +                     .v_flex()
    |

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
   --> src\latex_view.rs:153:30
    |
152 | /                         div()
153 | |                             .v_flex()
    | |_____________________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
    |
 73 |       fn v_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
    |
  7 + use gpui_component::StyledExt;
    |
help: there is a method `h_flex` with a similar name
    |
153 -                             .v_flex()
153 +                             .h_flex()
    |

error[E0599]: no method named `overflow_y_scroll` found for struct `gpui::Div` in the current scope
   --> src\latex_view.rs:169:38
    |
163 | / ...                   div()
164 | | ...                       .p_3()
165 | | ...                       .bg(rgb(0x181825))
166 | | ...                       .rounded(px(6.))
167 | | ...                       .text_color(rgb(0xa6e3a1))
168 | | ...                       .text_sm()
169 | | ...                       .overflow_y_scroll()
    | |___________________________-^^^^^^^^^^^^^^^^^
    |
help: there is a method `overflow_y_scrollbar` with a similar name
    |
169 |                                     .overflow_y_scrollbar()
    |                                                       +++

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
   --> src\latex_view.rs:177:30
    |
176 | /                         div()
177 | |                             .v_flex()
    | |_____________________________-^^^^^^
    |
   ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
    |
 73 |       fn v_flex(self) -> Self {
    |          ------ the method is available for `gpui::Div` here
    |
    = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
    |
  7 + use gpui_component::StyledExt;
    |
help: there is a method `h_flex` with a similar name
    |
177 -                             .v_flex()
177 +                             .h_flex()
    |

error[E0283]: type annotations needed
   --> src\latex_view.rs:304:56
    |
304 | fn create_typst_world(_source: &str) -> anyhow::Result<impl typst::World> {
    |                                                        ^^^^^^^^^^^^^^^^^ cannot infer type
...
308 |     todo!("Implement typst::World — use typst-as-lib crate for convenience")
    |     ------------------------------------------------------------------------ return type was inferred to be `!` here
    |
    = note: cannot satisfy `_: World`
    = help: the following types implement trait `World`:
              &W
              Box<W>
              std::sync::Arc<W>
              typst_library::_::__ComemoSurface<'__comemo_dynamic, '__comemo_tracked>
              typst_library::_::__ComemoSurfaceMut<'__comemo_dynamic, '__comemo_tracked>

error[E0599]: no method named `v_flex` found for struct `gpui::Div` in the current scope
  --> src\chart_view.rs:71:14
   |
70 | /         div()
71 | |             .v_flex()
   | |_____________-^^^^^^
   |
  ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:73:8
   |
73 |       fn v_flex(self) -> Self {
   |          ------ the method is available for `gpui::Div` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `v_flex` is implemented but not in scope; perhaps you want to import it
   |
 6 + use gpui_component::StyledExt;
   |
help: there is a method `h_flex` with a similar name
   |
71 -             .v_flex()
71 +             .h_flex()
   |

error[E0599]: no method named `h_flex` found for struct `gpui::Div` in the current scope
  --> src\chart_view.rs:84:22
   |
83 | /                 div()
84 | |                     .h_flex()
   | |_____________________-^^^^^^
   |
  ::: F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-component-0.5.1\src\styled.rs:67:8
   |
67 |       fn h_flex(self) -> Self {
   |          ------ the method is available for `gpui::Div` here
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `StyledExt` which provides `h_flex` is implemented but not in scope; perhaps you want to import it
   |
 6 + use gpui_component::StyledExt;
   |
help: there is a method `v_flex` with a similar name
   |
84 -                     .h_flex()
84 +                     .v_flex()
   |

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\chart_view.rs:109:40
     |
 109 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
 109 |                                     cx.notify(/* EntityId */);
     |                                               ++++++++++++++

error[E0061]: this method takes 1 argument but 0 arguments were supplied
    --> src\main.rs:96:40
     |
  96 | ...                   cx.notify();
     |                          ^^^^^^-- argument #1 of type `EntityId` is missing
     |
note: method defined here
    --> F:\DevCaches\cargo\registry\src\index.crates.io-1949cf8c6b5b557f\gpui-0.2.2\src\app.rs:2034:12
     |
2034 |     pub fn notify(&mut self, entity_id: EntityId) {
     |            ^^^^^^
help: provide the argument
     |
  96 |                                     cx.notify(/* EntityId */);
     |                                               ++++++++++++++

error[E0599]: no method named `overflow_y_scroll` found for struct `gpui::Div` in the current scope
   --> src\main.rs:106:22
    |
103 | /                 div()
104 | |                     .flex_1()
105 | |                     .p_4()
106 | |                     .overflow_y_scroll()
    | |_____________________-^^^^^^^^^^^^^^^^^
    |
help: there is a method `overflow_y_scrollbar` with a similar name
    |
106 |                     .overflow_y_scrollbar()
    |                                       +++

error[E0282]: type annotations needed
  --> src\video_view.rs:38:25
   |
38 |         cx.spawn(|this, mut cx| async move {
   |                         ^^^^^^
...
47 |                 cx.update(|cx| {
   |                 -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
38 |         cx.spawn(|this, mut cx: /* Type */| async move {
   |                               ++++++++++++

error[E0282]: type annotations needed
  --> src\video_view.rs:38:19
   |
38 |         cx.spawn(|this, mut cx| async move {
   |                   ^^^^
...
48 |                     this.update(cx, |view, cx| {
   |                     ---- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
38 |         cx.spawn(|this: /* Type */, mut cx| async move {
   |                       ++++++++++++

error[E0282]: type annotations needed
  --> src\video_view.rs:48:44
   |
48 |                     this.update(cx, |view, cx| {
   |                                            ^^
49 |                         view.loaded = true;
50 |                         cx.notify();
   |                         -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
48 |                     this.update(cx, |view, cx: /* Type */| {
   |                                              ++++++++++++

error[E0282]: type annotations needed
  --> src\video_view.rs:78:25
   |
78 |         cx.spawn(|this, mut cx| async move {
   |                         ^^^^^^
79 |             smol::Timer::after(frame_duration).await;
80 |             cx.update(|cx| {
   |             -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
78 |         cx.spawn(|this, mut cx: /* Type */| async move {
   |                               ++++++++++++

error[E0282]: type annotations needed
  --> src\video_view.rs:78:19
   |
78 |         cx.spawn(|this, mut cx| async move {
   |                   ^^^^
...
81 |                 this.update(cx, |view, cx| {
   |                 ---- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
78 |         cx.spawn(|this: /* Type */, mut cx| async move {
   |                       ++++++++++++

error[E0599]: no method named `data` found for struct `Packet` in the current scope
   --> src\video_view.rs:217:55
    |
217 |         if let Some(yuv) = h264_decoder.decode(packet.data())? {
    |                                                       ^^^^-- help: remove the arguments
    |                                                       |
    |                                                       field, not a method

error[E0599]: no method named `dimension_rgb` found for struct `DecodedYUV<'a>` in the current scope
   --> src\video_view.rs:218:39
    |
218 |             let (width, height) = yuv.dimension_rgb();
    |                                       ^^^^^^^^^^^^^
    |
help: there is a method `dimensions` with a similar name
    |
218 -             let (width, height) = yuv.dimension_rgb();
218 +             let (width, height) = yuv.dimensions();
    |

error[E0282]: type annotations needed
  --> src\three_d_view.rs:37:25
   |
37 |         cx.spawn(|this, mut cx| async move {
   |                         ^^^^^^
...
41 |                 cx.update(|cx| {
   |                 -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
37 |         cx.spawn(|this, mut cx: /* Type */| async move {
   |                               ++++++++++++

error[E0282]: type annotations needed
  --> src\three_d_view.rs:37:19
   |
37 |         cx.spawn(|this, mut cx| async move {
   |                   ^^^^
...
42 |                     this.update(cx, |_, cx| cx.notify())
   |                     ---- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
37 |         cx.spawn(|this: /* Type */, mut cx| async move {
   |                       ++++++++++++

error[E0282]: type annotations needed
  --> src\three_d_view.rs:42:41
   |
42 |                     this.update(cx, |_, cx| cx.notify())
   |                                         ^^  -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
42 |                     this.update(cx, |_, cx: /* Type */| cx.notify())
   |                                           ++++++++++++

error[E0282]: type annotations needed
  --> src\three_d_view.rs:63:25
   |
63 |         cx.spawn(|this, mut cx| async move {
   |                         ^^^^^^
...
67 |                 cx.update(|cx| {
   |                 -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
63 |         cx.spawn(|this, mut cx: /* Type */| async move {
   |                               ++++++++++++

error[E0282]: type annotations needed
  --> src\three_d_view.rs:63:19
   |
63 |         cx.spawn(|this, mut cx| async move {
   |                   ^^^^
...
68 |                     this.update(cx, |_, cx| cx.notify())
   |                     ---- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
63 |         cx.spawn(|this: /* Type */, mut cx| async move {
   |                       ++++++++++++

error[E0282]: type annotations needed
  --> src\three_d_view.rs:68:41
   |
68 |                     this.update(cx, |_, cx| cx.notify())
   |                                         ^^  -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
68 |                     this.update(cx, |_, cx: /* Type */| cx.notify())
   |                                           ++++++++++++

error[E0282]: type annotations needed
  --> src\pdf_view.rs:27:25
   |
27 |         cx.spawn(|this, mut cx| async move {
   |                         ^^^^^^
...
37 |                 cx.update(|cx| {
   |                 -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
27 |         cx.spawn(|this, mut cx: /* Type */| async move {
   |                               ++++++++++++

error[E0282]: type annotations needed
  --> src\pdf_view.rs:27:19
   |
27 |         cx.spawn(|this, mut cx| async move {
   |                   ^^^^
...
38 |                     this.update(cx, |view, cx| {
   |                     ---- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
27 |         cx.spawn(|this: /* Type */, mut cx| async move {
   |                       ++++++++++++

error[E0282]: type annotations needed
  --> src\pdf_view.rs:38:44
   |
38 |                     this.update(cx, |view, cx| {
   |                                            ^^
...
41 |                         cx.notify();
   |                         -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
38 |                     this.update(cx, |view, cx: /* Type */| {
   |                                              ++++++++++++

error[E0308]: mismatched types
   --> src\doc_view.rs:124:36
    |
124 | ...                   if let Some(ref rp) = run.run_property {
    |                              ^^^^^^^^^^^^   ---------------- this expression has type `docx_rs::RunProperty`
    |                              |
    |                              expected `RunProperty`, found `Option<_>`
    |
    = note: expected struct `docx_rs::RunProperty`
                 found enum `std::option::Option<_>`
help: you might have meant to use field `style` whose type is `std::option::Option<docx_rs::RunStyle>`
    |
124 |                             if let Some(ref rp) = run.run_property.style {
    |                                                                   ++++++

error[E0282]: type annotations needed
  --> src\latex_view.rs:60:25
   |
60 |         cx.spawn(|this, mut cx| async move {
   |                         ^^^^^^
...
73 |                     cx.update(|cx| {
   |                     -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
60 |         cx.spawn(|this, mut cx: /* Type */| async move {
   |                               ++++++++++++

error[E0282]: type annotations needed
  --> src\latex_view.rs:60:19
   |
60 |         cx.spawn(|this, mut cx| async move {
   |                   ^^^^
...
74 |                         this.update(cx, |view, cx| {
   |                         ---- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
60 |         cx.spawn(|this: /* Type */, mut cx| async move {
   |                       ++++++++++++

error[E0282]: type annotations needed
  --> src\latex_view.rs:74:48
   |
74 |                         this.update(cx, |view, cx| {
   |                                                ^^
75 |                             view.error_msg = None;
76 |                             cx.notify();
   |                             -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
74 |                         this.update(cx, |view, cx: /* Type */| {
   |                                                  ++++++++++++

error[E0282]: type annotations needed
  --> src\latex_view.rs:82:48
   |
82 |                         this.update(cx, |view, cx| {
   |                                                ^^
83 |                             view.error_msg = Some(e.to_string());
84 |                             cx.notify();
   |                             -- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
82 |                         this.update(cx, |view, cx: /* Type */| {
   |                                                  ++++++++++++

error[E0282]: type annotations needed
   --> src\latex_view.rs:229:9
    |
229 |     let document = typst::compile(&world)
    |         ^^^^^^^^
...
234 |     let page = document
    |                -------- type must be known at this point
    |
help: consider giving `document` an explicit type
    |
229 |     let document: /* Type */ = typst::compile(&world)
    |                 ++++++++++++

Some errors have detailed explanations: E0061, E0282, E0283, E0308, E0422, E0433, E0599.
For more information about an error, try `rustc --explain E0061`.
warning: `gpui-media-app` (bin "gpui-media-app") generated 2 warnings
error: could not compile `gpui-media-app` (bin "gpui-media-app") due to 55 previous errors; 2 warnings emitted
