/// Used to define a widget template, with properties and event handlers.
#[macro_export]
macro_rules! template {
    ($type:ident, [ $( $derive:ident ),* ]) => (

        use crate::{
            widget::TemplateBase,
            properties::{
                HorizontalAlignmentProperty,
                VerticalAlignmentProperty,
                EnabledProperty,
                VisibilityProperty,
                MarginProperty,
            },
            theme::SelectorProperty,
        };

        pub struct $type {
            template: Template
        }

        impl $type {
            /// Creates a new template.
            pub fn new() -> Self {
                $type {
                    template: Template::new()
                }
            }
        }

        impl From<Template> for $type {
            fn from(template: Template) -> Self {
                $type {
                    template
                }
            }
        }

        impl Into<Template> for $type {
            fn into(self) -> Template {
                self.template
            }
        }

        impl TemplateBase for $type {}

        impl HorizontalAlignmentProperty for $type {}

        impl VerticalAlignmentProperty for $type {}

        impl SelectorProperty for $type {}

        impl EnabledProperty for $type {}

        impl VisibilityProperty for $type {}

        impl MarginProperty for $type {}

        $(
            impl $derive for $type {}
        )*
    )
}

/// Used to define a property.
#[macro_export]
macro_rules! property {
    ($type:ident, $property:ident, $method:ident, $shared_method:ident) => {
        use dces::prelude::{Entity, EntityComponentManager};
        
        use crate::widget::{SharedProperty, Template, get_property, get_property_by_widget, has_property, WidgetContainer};

        pub trait $property: Sized + From<Template> + Into<Template> {
            /// Transforms the property into a template.
            fn template<F: FnOnce(Template) -> Template>(self, transform: F) -> Self {
                Self::from(transform(self.into()))
            }

            /// Inserts a property.
            fn $method<V: Into<$type>>(self, $method: V) -> Self {
                self.template(|template| template.property($method.into()))
            }

            /// Inserts a shared property.
            fn $shared_method(self, $method: SharedProperty) -> Self {
                self.template(|template| template.shared_property($method.into()))
            }
        }

        impl $type {
            pub fn get(entity: Entity, ecm: &EntityComponentManager) -> $type {
                get_property::<$type>(entity, ecm)
            }

            pub fn get_by_widget(widget: &WidgetContainer) -> $type {
                get_property_by_widget::<$type>(widget)
            }

            pub fn has(widget: &WidgetContainer) -> bool {
                has_property::<$type>(widget)
            }
        }
    };
}