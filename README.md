# Moo-Math

```bash
 _(_)_
'-e e-'
 (o_o)

What does the cow sound like?
It's moo~ moo moo
```

Mathematics interpreter utility, written without dependencies, capable of
- Interpreting normal mathematics equation
- Interpreting first order differential equation with Runge-Kutta method
- Can add custom math function

#### Example

```rust
fn main(){
    let mut moo = Moo::new(|functions| {
        // add custom function
        functions.insert("relu", |v| {
            f64::max(0.0, v)
        });
    });
    let program = moo.parse("x + 10 + relu(-6)").ok().unwrap().unwrap();
    // run(x) output: 10
    println!("{}", program.run(0.0));
}
```