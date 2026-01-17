use bevy_ecs::prelude::*;
// use bevy_transform::components::Transform;

/// Displays an entity hierarchy with component names.
#[must_use = "must be displayed"]
pub struct DebugEntityHierarchy<'w> {
    entity: Entity,
    world: &'w World,
}

pub fn debug_entity_hierarchy(entity: Entity, world: &World) -> DebugEntityHierarchy<'_> {
    DebugEntityHierarchy { entity, world }
}

impl std::fmt::Display for DebugEntityHierarchy<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entities = vec![(self.entity, String::new(), true)];

        while let Some((entity, mut prefix, last_child)) = entities.pop() {
            if !prefix.is_empty() {
                // From: https://en.wikipedia.org/wiki/Box-drawing_characters
                prefix.push(if last_child { '└' } else { '├' });
            }

            write!(f, "{}", prefix)?;

            match self.world.get::<Name>(entity) {
                Some(name) => write!(f, "\"{name}\" ({entity})")?,
                None => write!(f, "{entity}")?,
            }

            let mut components = self
                .world
                .inspect_entity(entity)
                .unwrap()
                // .into_iter()
                .map(|c| c.name());

            if let Some(c) = components.next() {
                write!(f, ": [{}", c.shortname())?;

                for c in components {
                    write!(f, ", {}", c.shortname())?;
                }

                write!(f, "]")?;
            }

            // if let Some(v) = self.world.get::<InheritedVisibility>(entity) {
            //     write!(f, " visible: {}", v.get())?;
            // }
            // if let Some(t) = self.world.get::<Transform>(entity) {
            //     write!(f, " t: {:?}", t.forward())?;
            // }
            // if let Some(gt) = self.world.get::<GlobalTransform>(entity) {
            //     write!(f, " gt: {:?}", gt.translation())?;
            // }

            writeln!(f)?;

            prefix.pop();

            if let Some(children) = self.world.get::<Children>(entity) {
                assert!(!children.is_empty(), "children is never empty");
                prefix.push(if last_child { ' ' } else { '│' });

                match children.split_last() {
                    Some((last_child, rest)) => {
                        // Entities are popped from the end, so we reverse the order.
                        entities.push((*last_child, prefix.clone(), true));
                        entities.extend(rest.iter().rev().map(|c| (*c, prefix.clone(), false)));
                    }
                    None => unreachable!(),
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_entity_hierarchy() {
        let mut world = World::default();
        let root = world
            .spawn(Name::new("root"))
            .with_children(|p| {
                p.spawn(Name::from("child_a")).with_children(|p| {
                    p.spawn(Name::from("child_c"));
                    p.spawn(Name::from("child_d"))
                        .with_children(|p| _ = p.spawn(Name::from("child_f")));
                });
                p.spawn(Name::from("child_b"))
                    .with_children(|p| _ = p.spawn(Name::from("child_e")));
            })
            .id();

        let displayed = format!("{}", debug_entity_hierarchy(root, &mut world));

        let expected = r#""root" (0v0): [Name, Children]
 ├"child_a" (1v0): [Name, ChildOf, Children]
 │├"child_c" (2v0): [Name, ChildOf]
 │└"child_d" (3v0): [Name, ChildOf, Children]
 │ └"child_f" (4v0): [Name, ChildOf]
 └"child_b" (5v0): [Name, ChildOf, Children]
  └"child_e" (6v0): [Name, ChildOf]
"#;

        assert_eq!(displayed, expected);
    }
}
