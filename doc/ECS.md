# Entity-Component-System (ECS) Architecture

## Overview

The **Entity-Component-System (ECS)** is a software architectural pattern commonly used in game development and simulations to achieve a modular, scalable, and efficient approach to managing game objects and their behaviors.

### Key Concepts

1. **Entity**
   - An **entity** is a unique identifier that represents any object or actor in the system (e.g., player, enemy, projectile).
   - It has no data or behavior by itself, but serves as a container for components.

2. **Component**
   - A **component** is a data structure that holds specific attributes or properties of an entity (e.g., position, velocity, health, or appearance).
   - Components are responsible for storing data, not behavior. Each component focuses on a single aspect of the entity's state.

3. **System**
   - A **system** contains the logic that operates on entities based on their components.
   - Systems process all entities that possess certain components, applying game logic or updates to their state (e.g., a system might move entities with position and velocity components).

4. **Resources**
   - "globally unique" data of some kind. In Bevy ECS, we represent globally unique data using the Resource trait.

Here are some examples of data that could be encoded as a Resource:

Elapsed Time
Asset Collections (sounds, textures, meshes)
Renderers
### How ECS Works

- **Entities** act as containers, holding one or more components.
- **Components** define data relevant to specific attributes of the entity.
- **Systems** query for entities that have the required components and perform updates or computations based on those components.

This separation of concerns improves flexibility, allowing entities to have different sets of components, and makes it easier to extend or change behaviors without tightly coupling data and logic.

### Example

1. An entity representing a player might have:
   - `PositionComponent`
   - `VelocityComponent`
   - `PlayerComponent`

2. A **movement system** might update the entity’s `PositionComponent` based on the `VelocityComponent`:

   ```rust
   fn movement_system(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
       for (mut position, velocity) in query.iter_mut() {
           position.x += velocity.x * time.delta_seconds();
           position.y += velocity.y * time.delta_seconds();
       }
   }
