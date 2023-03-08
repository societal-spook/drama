#[cfg(test)]
mod tests {
    use drama::*;
    use drama_proc_macros::{system};

    #[derive(Component)]
    struct Position {
        x: f32,
    }

    #[derive(Component)]
    struct Velocity {
        v: f32,
    }

    #[system]
    fn hello() {}


    #[test]
    fn it_works() {
        let vel = Velocity { v: 123.0 };

        let entity = entity!(Velocity { v: 123.0 });

        let scene = Scene::new();

        assert!(hello() == 42)
    }
}