pub mod components;

pub use drama_proc_macros::{Component};
pub use components::Component;

pub struct Scene {
    entities: Vec<Entity>
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new()
        }
    }

    pub fn add(self: &mut Self, entity: Entity) {
        self.entities.push(entity)
    }
}

pub struct System {
  name: String
}

impl System {
    pub fn new() -> Self {
        return Self {
            name: "123".parse().unwrap()
        }
    }
}

pub struct Entity {
    components: Vec<Box<dyn Component>>
}

impl Entity {
    pub fn new(components: Vec<Box<dyn Component>>) -> Self {
        Self {
            components
        }
    }
}

#[macro_export]
macro_rules! entity {
    ( $( $x:expr ),* ) => {
        {
            let mut components = Vec::<Box<dyn Component>>::new();
            $(
                components.push(Box::new($x));
            )*

            Entity::new(components)
        }
    };
}
