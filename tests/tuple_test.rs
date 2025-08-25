use std::collections::HashMap;

use cucumber::{World, given, then};

use rrt::Tuple;

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct TupleWorld {
    t: Tuple,
    tmap: HashMap<String, Tuple>,
}
impl TupleWorld {
    fn get_at(&self, idx: String) -> &Tuple {
        &self.tmap[&idx]
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("a hungry cat")]
fn hungry_cat(world: &mut TupleWorld) {}

#[given(expr = "{word} = tuple\\({float}, {float}, {float}, {float}\\)")]
fn tuple_created(world: &mut TupleWorld, idx: String, x: f32, y: f32, z: f32, w: f32) {
    world.tmap.insert(idx, Tuple { x, y, z, w });
}

#[given(expr = "{word} = point\\({float}, {float}, {float}\\)")]
fn point_created(world: &mut TupleWorld, idx: String, x: f32, y: f32, z: f32) {
    world.tmap.insert(idx, Tuple::point(x, y, z));
}

#[given(expr = "{word} = vector\\({float}, {float}, {float}\\)")]
fn vector_created(world: &mut TupleWorld, idx: String, x: f32, y: f32, z: f32) {
    world.tmap.insert(idx, Tuple::vector(x, y, z));
}

#[then(expr = "{word}.x = {float}")]
fn check_x(world: &mut TupleWorld, idx: String, expected: f32) {
    if world.tmap[&idx].x != expected {
        panic!();
    }
}

#[then(expr = "{word}.y = {float}")]
fn check_y(world: &mut TupleWorld, idx: String, expected: f32) {
    if world.tmap[&idx].y != expected {
        panic!();
    }
}

#[then(expr = "{word}.z = {float}")]
fn check_z(world: &mut TupleWorld, idx: String, expected: f32) {
    if world.tmap[&idx].z != expected {
        panic!();
    }
}

#[then(expr = "{word}.w = {float}")]
fn check_w(world: &mut TupleWorld, idx: String, expected: f32) {
    if world.tmap[&idx].w != expected {
        panic!();
    }
}

#[then(expr = "{word} is a point")]
#[then(expr = "{word} is not a vector")]
fn check_is_point(world: &mut TupleWorld, idx: String) {
    let v = world.tmap[&idx];
    if !Tuple::is_point(&v) || Tuple::is_vector(&v) {
        panic!();
    }
}

#[then(expr = "{word} is not a point")]
#[then(expr = "{word} is a vector")]
fn check_is_vector(world: &mut TupleWorld, idx: String) {
    let v = world.tmap[&idx];
    if Tuple::is_point(&v) || !Tuple::is_vector(&v) {
        panic!();
    }
}

#[then(expr = "equal\\({word}, {word}\\)")]
fn check_eq(world: &mut TupleWorld, lhs: String, rhs: String) {
    let a = world.tmap[&lhs];
    let b = world.tmap[&rhs];
    if !(a == b) {
        panic!()
    }
}
#[then(expr = "not equal\\({word}, {word}\\)")]
fn check_ineq(world: &mut TupleWorld, lhs: String, rhs: String) {
    let a = world.tmap[&lhs];
    let b = world.tmap[&rhs];
    if a == b {
        panic!()
    }
}

#[then(expr = "{word} = {word} + {word}")]
fn add_tuple(world: &mut TupleWorld, idx: String, lhs: String, rhs: String) {
    let a = world.tmap[&lhs];
    let b = world.tmap[&rhs];
    let expected = world.tmap[&idx];
    let actual = a + b;
    if actual != expected {
        panic!("{actual:?} was not {expected:?}");
    }
}

#[then(expr = "{word} = {word} - {word}")]
fn sub_tuple(world: &mut TupleWorld, idx: String, lhs: String, rhs: String) {
    let a = world.tmap[&lhs];
    let b = world.tmap[&rhs];
    let expected = world.tmap[&idx];
    let actual = a - b;
    if actual != expected {
        panic!("{actual:?} was not {expected:?}");
    }
}

#[then(expr = "{word} negated is {word}")]
fn neg_tuple(world: &mut TupleWorld, idx: String, lhs: String) {
    let a = world.tmap[&idx];
    let b = world.tmap[&lhs];
    let actual = -a;
    if actual != b {
        panic!("{actual:?} was not {b:?}");
    }
}

#[then(expr = "{word} \\/ {int} = {word}")]
fn divide_tuple(world: &mut TupleWorld, idx: String, scalar: i32, lhs: String) {
    let a = world.tmap[&idx];
    let b = world.tmap[&lhs];
    let actual = a / scalar;
    if actual != b {
        panic!("{actual:?} was not {b:?}");
    }
}

#[then(expr = "{word} \\/ {float}f = {word}")]
fn divide_tuple_f(world: &mut TupleWorld, idx: String, scalar: f32, lhs: String) {
    let a = world.tmap[&idx];
    let b = world.tmap[&lhs];
    let actual = a / scalar;
    if actual != b {
        panic!("{actual:?} was not {b:?}");
    }
}

#[then(expr = "{word} * {int} = {word}")]
fn mult_tuple(world: &mut TupleWorld, idx: String, scalar: i32, lhs: String) {
    let a = world.tmap[&idx];
    let b = world.tmap[&lhs];
    let actual = a * scalar;
    if actual != b {
        panic!("{actual:?} was not {b:?}");
    }
}

#[then(expr = "{word} * {float}f = {word}")]
fn mult_tuple_frac(world: &mut TupleWorld, idx: String, scalar: f32, lhs: String) {
    let a = world.tmap[&idx];
    let b = world.tmap[&lhs];
    let actual = a * scalar;
    if actual != b {
        panic!("{actual:?} was not {b:?}");
    }
}

#[then(expr = "magnitude\\({word}\\) = {float}")]
fn mag(world: &mut TupleWorld, idx: String, expected: f32) {
    let actual = world.tmap[&idx].magnitude();
    if actual != expected {
        panic!("{actual} was not {expected}");
    }
}

#[then(expr = "magnitude\\({word}\\) = √{float}")]
fn mag_sqrt(world: &mut TupleWorld, idx: String, expected: f32) {
    let actual = world.tmap[&idx].magnitude();
    let sqactual = actual * actual;
    assert_float_eq::assert_f32_near!(sqactual, expected);
}

// This runs before everything else, so you can set up things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TupleWorld::run("tests/features/rtt/tuple.feature"));
}
