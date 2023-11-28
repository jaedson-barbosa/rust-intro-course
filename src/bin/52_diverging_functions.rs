/*
Diverging functions never return. They are marked using !, which is an empty type.
As opposed to all the other types, this one cannot be instantiated, because the set of all possible values this type can have is empty.
Note that, it is different from the () type, which has exactly one possible value.
*/

fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    let x = foo();
    println!("You will never see this line!");
}
