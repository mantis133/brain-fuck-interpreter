# BrainFuck + (WIP)

This is an interpreter for my custom implementation of the programming "language" BrainFuck. It has the ability to read the standard<sup>[1](#standard-brainfuck)</sup> implementation of BrainFuck when configured correctly<sup>[2](#brainfuck--wip)</sup>, however it can also read in some custom variations I have made because this "language" clearly wasn't cursed enough.


## <sup>[1](#standard-brainfuck)</sup>Standard BrainFuck

The BrainFuck Programming "Language" is a simple languge that gives you an array consisting of 30_000 byte/u8 values and a pointer to the left-most/0th index position. The values inside of the array can be manipulated using the following keywords:

| Keyword. | What It Does. |
| ---- | ---- |
| **+** | Increments the value in the cell the data pointer in pointing to. |
| **-** | decrements the value in the cell the data pointer in pointing to. |
| **>** | moves the data pointer one cell to the right. |
| **<** | Moves the data pointer one cell to the left. |
| **.** | prints the value of the currently pointed to cell as ascii (without a new line character) to stdout. |
| **,** | pauses the program and accepts input from stdin. |
| **[** | If the current pointed to data cell is zero, jump to the instruction following the matching closing bracket and continue, else continue. |
| **]** | If the current pointed to data cell is **non zero** jump back to corresponding opening bracket else if current pointed to cell is zero continue to next instruction. |
| **_** | All other characters are ignored to be treated as comments. |



BrainFuck "programs" are written in .bf files or if you want to use a .bff file insert the following code in the first row and place the rest of the program below.
```BrainForeverFucked
30_000 - u8 - s
``` 