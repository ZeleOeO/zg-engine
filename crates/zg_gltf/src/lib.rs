mod model;

pub use model::Model;

//  Model::load(world, location);
// if i have multiple meshes and materials, they can all be their own entity
// which was the point of having world.spawn() in here...
//
// for each material and mesh combo, we spawn it and return maybe a vector of entities?
// i doubbt it would be that smooth but let's see
