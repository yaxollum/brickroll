# brickroll
A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) to [Rickroll](https://github.com/BattleMage0231/rickroll) compiler

## Example
The following Brainfuck program takes a single character as input and outputs the next ASCII character:
```
,+.>++++++++++.
```

You can save it as a file called `Input.bf` and compile it using `cargo run Input.bf -o Input.rickroll`. Now you can run the Rickroll program in the standard way: `rickroll Input.rickroll`.
