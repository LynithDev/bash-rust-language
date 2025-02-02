# Another "scripting language"
This one will transpile to various targets (BASH, PowerShell etc.)

```js
var my_variable = 512

fn awesome(name: String): String {
    $echo "Hello #{name}!"
    
    $echo hello world

    return $echo this gets returned to the function explicitly
}

$echo "return value: #{awesome("Rust")}"
```

```js
fn add(a: Int, b: Int): Int {
    a + b # implicit return
}

for i in 0..=5 {
    // prints '1 + 2 = 3', '2 + 3 = 4'...
    $echo "#{i} + #{i + 1} = #{add(i, i + 1)}
}
```

```js
$read(-p "login: ") // sets the $REPLY variable

var input = REPLY // shell variables are treated like normal variables
var msg = match input {
    "Admin" => "Access granted."
    * => "Denied."
}

$echo #{msg} // templating
```