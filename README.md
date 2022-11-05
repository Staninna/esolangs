<!-- Disable markdown lint warnings -->
<!-- markdownlint-disable MD033 -->

# Esolangs

A esolang is a programming language that is not intended to be used for practical purposes. They are often used as a joke, educational tool, or as a challenge to write programs in. [here](https://esolangs.org/wiki/Language_list) is a list with most of them.

This repository is a collection of esolangs interpreters.

## List of esolangs in this repository

| Name                                                   | Description                                                | Goto readme                |
| ------------------------------------------------------ | ---------------------------------------------------------- | -------------------------- |
| [Brainfuck](https://esolangs.org/wiki/Brainfuck)       | A simple esolang that uses only 8 instructions.            | [README.md](#brainfuck)    |
| [Spyrodecimal](https://esolangs.org/wiki/Spyrodecimal) | It was designed to have only numbers as instructions.      | [README.md](#spyrodecimal) |
| [Barely](https://esolangs.org/wiki/Barely)             | The goal is to have as small of a interpreter as possible. | [README.md](#barely)       |

### Brainfuck

Brainfuck is a simple esolang that uses only 8 instructions. It is Turing complete and one-dimensional.
<br>
The instructions are:

| Command | Description                                                                                                                                                                                   |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `>`     | Increment the data pointer (to point to the next cell to the right).                                                                                                                          |
| `<`     | Decrement the data pointer (to point to the next cell to the left).                                                                                                                           |
| `+`     | Increment (increase by one) the byte at the data pointer.                                                                                                                                     |
| `-`     | Decrement (decrease by one) the byte at the data pointer.                                                                                                                                     |
| `.`     | Output the byte at the data pointer.                                                                                                                                                          |
| `,`     | Accept one byte of input, storing its value in the byte at the data pointer.                                                                                                                  |
| `[`     | If the byte at the data pointer is zero, <br> then instead of moving the instruction pointer forward to the next command, <br> jump it forward to the command after the matching `]` command. |
| `]`     | If the byte at the data pointer is nonzero, <br> then instead of moving the instruction pointer forward to the next command, <br> jump it back to the command after the matching `[` command. |
| `#`     | Print the current memory state from the data pointer to data pointer + 16                                                                                                                     |

Hello World:

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

### Spyrodecimal

Spyrodecimal is a esolang that was designed to have only numbers as instructions but it knows also some letters as instructions. It is Turing complete and one-dimensional. Made by [Spyro543](https://www.cemetech.net/forum/viewtopic.php?t=7250).
<br>
The instructions are:

| Command | Description                                                                                              |
| ------- | -------------------------------------------------------------------------------------------------------- |
| `0`     | Pauses for 1/10 seconds                                                                                  |
| `1`     | Outputs the ASCII byte in memory                                                                         |
| `2`     | Increase memory                                                                                          |
| `3`     | Decrease memory                                                                                          |
| `4`     | Store input in memory                                                                                    |
| `5`     | Print new line                                                                                           |
| `6`     | generate random number between 1 and 256 and store it in memory                                          |
| `7`     | Moves the program pointer back the amount of times the memory is                                         |
| `8`     | Resets memory to 0                                                                                       |
| `9`     | Moves the program pointer forward the amount of times the memory                                         |
| `q`     | Quits the interpreter                                                                                    |
| `x`     | Quits the program                                                                                        |
| `s`     | Stores memory in 1 of 6 variables (`a`, `b`, `c`, `d`, `e`, `f`). <br> Variables are not effected by `8` |
| `r`     | Loads memory from 1 of 6 variables (`a`, `b`, `c`, `d`, `e`, `f`) into memory                            |

Hello, World:

```spyrodecimal
22222222222222222222222222222222sc22222222222222222222222222222222222222221sd
3331se
222222211sa
2221sb8
rc2222222222221
rc1
rd2222222222222221
rb1
2221
ra1
re31
```

### Barely

Barely is a esolang that had the goal to have as small of a sturing complete interpreter as possible. And the original was just 59 bytes in size. it is Turing complete and one-dimensional.
<br>
The instructions are:

| Command                        | Description                                  |
| ------------------------------ | -------------------------------------------- |
| `]`                            | Exits program                                |
| `^`                            | If accumulator is 0, execute `b` instruction |
| `b`                            | Add jump to ip                               |
| `g`                            | Accumulator is memory                        |
| `h`                            | Add 71 to accumulator                        |
| `i`                            | Increment memory pointer                     |
| `j`                            | Increment accumulator                        |
| `k`                            | Decrement jump                               |
| `m`                            | memory is accumulator                        |
| `n`                            | Decrement memory pointer                     |
| `o`                            | Decrement accumulator                        |
| `p`                            | Add 10 to jump                               |
| `t`                            | Store user input in accumulator              |
| `x`                            | Output accumulator                           |
| `f` \|\| `l` \|\| `q` \|\| `s` | No operation                                 |
| `~`                            | No operation also marks start of program     |

Hello World:

```barely
]xhhhhooooooooohhhhhhxooooooooxooooooxjjjxhoooohhhxhohhhhhhhxhhhhjjjhhhxhhhhooooooooohhhhhhxjjjxxjjjjjjjxjhhhhxjhhhhhhhhjjjhh~
```
