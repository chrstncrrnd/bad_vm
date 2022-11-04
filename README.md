# BAD_VM

a register-based vm built in rust

## Usage:

(Make sure you have [Rust](https://www.rust-lang.org/) installed)

Run a file using ```cargo run <filename>```

Build a file from badvm assembly to bytecode using assembler: ```cargo run -p assembler -- <input> <output>```

### Opcodes:
- **HALT** (0)
- **LOAD** (1)
- **ADD** (2)
- **SUB** (3)
- **MUL** (4)
- **DIV** (5)

#### HALT
Stops program execution

#### LOAD
**Syntax:**

``` LOAD $0 <val> ```

Loads value `<val>` into register `$0`

#### ADD
**Syntax:**

``` ADD $0 $1 $2 ```

Adds the value in register `$0` to value in register `$1` and stores the result in register `$2`

#### SUB
**Syntax:**

``` SUB $0 $1 $2 ```
Substracts the value in register `$1` from value in register `$0` and stores the result in register `$2`


#### MUL
**Syntax:**

``` MUL $0 $1 $2 ```
Multiplies the value in register `$0` with value in register `$1` and stores the result in register `$2`


#### DIV
**Syntax:**

``` DIV $0 $1 $2 ```
Divides the value in register `$0` by value in register `$1` and stores the result in register `$2` and remainder in the remainder register

