//! The `belly` is a plugin for a [Bevy](https://bevyengine.org/) engine that
//! helps to declaratively define a user interface with `eml` markup (macros & asset),
//! style it with a very CSS-like `ess` syntax, and define data flow using `from!` &
//! `to!` bind macros and/or connect to signals (events) with `connect!` macro.
//!
//! The project is at the early stage of development, pretty much experimental,
//! API is unstable and will change in future.
//!
//! The project documentations grows slowly, the best source of knowlage at
//! the moment is the [README](https://github.com/jkb0o/belly#about) and
//! [examples](https://github.com/jkb0o/belly/tree/main/examples).
//!
//! ### Crate
//! The `belly` crate is just container crate that makes it easier to consume subcrates.
//! It has to main mods: `prelude` for using plugin and `build` for extending plugin.
//!
#![doc = ::embed_doc_image::embed_image!("color_picker", "docs/img/examples/color-picker.gif")]



pub use belly_core as core;
pub use belly_widgets as widgets;

/// `use belly::prelude::*` for adding the UI to your project
pub mod prelude {
    use belly_core::ElementsCorePlugin;
    use belly_widgets::WidgetsPlugin;
    use bevy::prelude::*;

    pub use belly_core::prelude::*;
    pub use belly_macro::eml;
    pub use belly_macro::ess;
    pub use belly_macro::run;
    pub use belly_widgets::prelude::*;

    pub struct BellyPlugin;
    impl Plugin for BellyPlugin {
        fn build(&self, app: &mut App) {
            app.add_plugins(ElementsCorePlugin);
            app.add_plugins(WidgetsPlugin);
        }
    }
}

/// `use belly::build::*` for extending the `belly` plugin with custom elements & styles
pub mod build {
    pub use super::prelude::*;
    pub use belly_core::build::*;
    pub use belly_macro::widget;
}
