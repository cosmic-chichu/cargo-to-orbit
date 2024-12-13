fn main() {
    println!("Hello, world!");
    let result = reverse("foo");
    println!("{}", result);
}

fn reverse(input: &str) -> String {
    let foo: String = input.chars().rev().collect();

    println!("{}", foo);


    input.to_string()
}