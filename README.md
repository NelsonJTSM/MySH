# MySH
MySH is a custom shell written in Rust.

## How to run
Install [Rust](https://www.rust-lang.org/tools/install) and Cargo.

```
git clone https://github.com/NelsonJTSM/MySH
cd MySH
cargo run
```

## How to use

MySH has certain functions similar to ```/bin/bash```, these include.

### Move to Directory

```
movetodir [directory]
```

Which moves to the given directory.

### Where Am I

```
whereami
```

Which displays the directory the shell is currently in.

### History

```
history
```

Which prints the history of commands used.

If the ```-c``` flag is passed, it clears the history.

### Bye Bye

```
byebye
```

Terminates the MySH shell.

### Start

```
start [program] [args]
```

Executes the given program. 

Currently only uses full path.

### Background

```
background [program] [args]
```

Executes the given program and puts it in the background.

Outputs: The process' PID.

### Exterminate

```
exterminate [PID]
```

Immediately terminates the program with the specific PID.

## TODO Commands

### Repeat

```
repeat [n] [command] ...
```

Runs the given shell command ```n``` amount of times.

### Exterminate All

```
exterminateall
```

Exterminates all the programs started by MySH.