<!-- Disable markdown lint warnings -->
<!-- markdownlint-disable MD033 -->

# Esolangs

A esolang is a programming language that is not intended to be used for practical purposes. They are often used as a joke, educational tool, or as a challenge to write programs in. This repository is a collection of esolangs interpreters.

## List of esolangs in this repository

| Name                                             | Description                                 | Goto readme             |
| ------------------------------------------------ | ------------------------------------------- | ----------------------- |
| [Brainfuck](https://esolangs.org/wiki/Brainfuck) | A simple esolang that uses only 8 commands. | [README.md](#brainfuck) |

### Brainfuck

Brainfuck is a simple esolang that uses only 8 commands. It is Turing complete machine.
<br>
The instructions are:

| Command | Description                                                                                                                                                                         |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `>`     | Increment the data pointer (to point to the next cell to the right).                                                                                                                |
| `<`     | Decrement the data pointer (to point to the next cell to the left).                                                                                                                 |
| `+`     | Increment (increase by one) the byte at the data pointer.                                                                                                                           |
| `-`     | Decrement (decrease by one) the byte at the data pointer.                                                                                                                           |
| `.`     | Output the byte at the data pointer.                                                                                                                                                |
| `,`     | Accept one byte of input, storing its value in the byte at the data pointer.                                                                                                        |
| `[`     | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `]` command. |
| `]`     | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `[` command. |
| `#`     | Print the current memory state from the data pointer to data pointer + 16                                                                                                           |

Hello World:

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```
