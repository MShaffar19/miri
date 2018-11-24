// Make sure that we cannot pass by argument a `&` that got already invalidated.
fn foo(_: &i32) {}

fn main() {
    let x = &mut 42;
    let xraw = &*x as *const _ as *mut _;
    let xref = unsafe { &*xraw };
    unsafe { *xraw = 42 }; // unfreeze
    foo(xref); //~ ERROR is not frozen
}