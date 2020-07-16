<img alt="OrbTk" width="380" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/logos/orbtk/logo_dark.png">

[![Build and test](https://github.com/redox-os/orbtk/workflows/build/badge.svg)](https://github.com/redox-os/orbtk/actions)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.3.1alpha2-orange.svg)](https://crates.io/crates/orbtk/0.3.1-alpha2)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/crate/orbtk/0.3.1-alpha2)

The Orbital Widget Toolkit is a cross-platform (G)UI toolkit for building scalable user interfaces with the programming language Rust. It's based
on the [Entity Component System Pattern](https://en.wikipedia.org/wiki/Entity_component_system) and provides a [functional Reactive](https://en.wikipedia.org/wiki/Functional_reactive_programming)-like API.

The main goals of OrbTk are speed, ease of use, and cross-platform compatibility.

<img alt="Calculator" width="350" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Features:

* Modern lightweight API
* Cross platform
* Modular crates
* Based on Entity Component System library [DCES](https://gitlab.redox-os.org/redox-os/dces-rust)
* Flexible event system
* Integrated widget library
* Custom widgets
* Theming
* Integrated debugging tools

## Platforms

* Redox OS (native)
* Linux (native | cargo-node)
* macOS (native | cargo-node)
* Windows (native | cargo-node)
* openBSD (not tested, but should work)
* Web (cargo-node)
* Android (native planned | cargo-node)
* iOS (native planned | cargo-node planned)
* Ubuntu Touch (native planned | cargo-node planned)

## Planned features

* Conformable use of async
* More default widgets
* More examples
* Book
* Animations
* Split application in modules
* Theme update
* 3D context
* More integrated debugging tools


## Usage

To include OrbTk in your project, add this dependency
line to your `Cargo.toml` file:

```text
orbtk = "0.3.1-alpha2"
```

To use the latest development version of OrbTk, add this dependency
line to your `Cargo.toml` file:

```text
orbtk = { git = "https://github.com/redox-os/orbtk.git", branch = "develop" }
```
You can also check out the OrbTk template project to start a new project: https://github.com/redox-os/orbtk-template.

## Minimal Example

```rust
use orbtk::prelude::*;

fn main() {
      Application::new()
        .window(|ctx| {
            Window::new()
                .title("OrbTk - minimal example")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(TextBlock::new().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
```

## Base concepts

### Widget

Widgets are the building blocks of user interfaces in OrbTk. They are things like Buttons, TextBoxes, ListViews, Views (Screens) and Grid(Layout)s. Each widget implements the [Widget trait](https://github.com/redox-os/orbtk/blob/develop/crates/api/src/widget/mod.rs) and is generated by the [widget! macro](https://github.com/redox-os/orbtk/blob/develop/crates/api/src/macros.rs). A widget consists of a name like `Button` and a list of its properties like `text: String16`, `background: Brush` or `count: u32`. After the `build` method of a widget is called it's added to the Entity Component System where it exists as an `Entity` (index) with `Components`. The struct of a widget serves as a builder using the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).

Basic usage of the widget! macro:

```rust
widget!(
    MyWidget {
      background: Brush,
      count: u32,
      text: String16,
      ...
    }
);
```

### Widget Templates

Each widget has to implement the [Template trait](https://github.com/redox-os/orbtk/blob/develop/crates/api/src/widget/template.rs). The template defines the default values of a widget's properties as well as its structure. A
Button e.g. consists of a `Container` widget, a `StackPanel` widget and a `TextBlock` widget.

Basic usage of the Template trait:

```rust
impl Template for MyWidget {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
         self.name("MyWidget")
            .background("#000000")
            .count(0)
            .text("Initial text")
            .child(Container::new()
                    // Container references the same background as MyWidget
                    .background(id)
                    .child(TextBlock::new()
                            // TextBlock references the same text as MyWidget
                            .text(id)
                            .build(ctx)
                    )
                    .build(ctx)
            )
    }
}
```

### Widget State

The state of a widget is used to update its inner state. Each state has to implement the [State trait](https://github.com/redox-os/orbtk/blob/develop/crates/api/src/widget/state.rs). The inner state of a widget is represented by the 
current values of its properties.

Basic usage of the state trait:

```rust
#[derive(Default, AsAny)]
struct MyState {
    ...
}

impl State for MyState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context) {
        // update the widget
        ...
    }
}

widget!(
    // Add MyState as state of MyWidget
    MyWidget<MyState> {
        ...
    }
);
```

The [Context parameter](https://github.com/redox-os/orbtk/blob/develop/crates/api/src/widget/context.rs) of the update method provides access to the state's widget (entity) and its properties (components). It also provides functions to access the children of the widget, and to manipulate the widget tree.

## Run Examples

You can find examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets --release
```

OrbTk has integrated `debug` tools. If you want to show the bounds of all widgets (even invisible ones) and want to see a debug print of the whole widget tree, you can run the examples with `--features debug`, like this:

```text
cargo run --example widgets --release --features debug
```

## Run Examples with cargo-node

To run the examples as a browser, electron or cordova app you have to install cargo-node:

```text
cargo install -f cargo-node
```

Before you can use cargo-node you have to install `npm` version 6.9.0, which is included with `Node.js` version 10.16.3. You can download it from https://nodejs.org/dist/v10.16.3/. 

Rust's `cargo` is also required. The rest of cargo-node's dependencies are installed automatically.

### Start examples

You can run the "widgets" example by executing one of the following commands:

* Run as browser app:

```text
cargo node run --target browser --example widgets
```

* Run as electron app:

```text
cargo node run --target electron --example widgets
```

* Run as cordova app on android:

```text
cargo node run --target android --example widgets
```

## Build and run documentation

You can build and view the latest documentation by executing the following command:

```text
cargo doc --no-deps --open
```

## Sub Crates

* [api](https://github.com/redox-os/orbtk/tree/develop/crates/api): base api elements of OrbTk e.g. widget and application parts
* [css-engine](https://github.com/redox-os/orbtk/tree/develop/crates/css-engine): parse and read values from a css file
* [proc-macros](https://github.com/redox-os/orbtk/tree/develop/crates/proc-macros): procedural helper macros
* [render](https://github.com/redox-os/orbtk/tree/develop/crates/render): cross platform 2D/3D render library
* [shell](https://github.com/redox-os/orbtk/tree/develop/crates/api): cross platform window and event handling
* [theme](https://github.com/redox-os/orbtk/tree/develop/crates/theme): OrbTks default theme (light and dark)
* [tree](https://github.com/redox-os/orbtk/tree/develop/crates/tree): tree structure based on DCES
* [utils](https://github.com/redox-os/orbtk/tree/develop/crates/utils): helper structs and traits
* [widgets](https://github.com/redox-os/orbtk/tree/develop/crates/widgets): base widget library

## Inspirations

* [Flutter](https://flutter.io/)
* [React](https://reactjs.org/)
* [Yew](https://github.com/DenisKolodin/yew)

## Showcases

* [Space Editor](https://codeberg.org/flovanco/space-editor): 2D Tile Map Editor compatible with OrbGame
* [doit](https://codeberg.org/flovanco/doit): Task app
* [OrbCalculator](https://gitlab.redox-os.org/redox-os/orbcalculator): Calculator based on OrbTk
* [Kanter](https://github.com/lukors/kanter): Node based texture editor 
* [twin-commander](https://github.com/kivimango/twin-commander): A twin-panel file manager specifically for the Redox OS 

## Contribution

If you want to help improve OrbTk you submit your feedback in the issue tracker, or make a pull request to fix an issue https://github.com/redox-os/orbtk/issues. You can also discuss OrbTk with us on the Redox chat https://redox-os.org/community/ (join the OrbTk channel).

## License

Licensed under MIT license ([LICENSE](LICENSE)).
