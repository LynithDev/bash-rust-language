# another "scripting language" lol
this one will transpile to bash

```ruby
var my_variable = 512

fn awesome(name: String): String {
    $echo "Hello {name}!"
    
    $echo hello world

    return $echo(this gets returned to the function explicitly)
}

$echo "return value: {awesome("Rust")}"

fn add(a: Int, b: Int): Int {
    a + b # implicit return
}

$echo(5 + 2 = { add(5, 2) }) # prints '5 + 2 = 7'
```