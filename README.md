An esolang based on graph theory.

# Syntax

## Pointer
s
A variable that points to a node.

| Value | Type | Description |
| --- | --- | --- |
| a .. z | Regular Pointer | Mutable and explicit pointer to a possible node |
| _ | Null Pointer | Constant null |
| ! | New Pointer | Always will point to a node that is not referenced by any other node or pointer, creates a node if necessary |
| ( | Input Pointer | Will take a byte of input and make a chain of that length as soon as this pointer is referenced in a statement, the pointer would point to the first node of the chain |
| ) | Output Pointer | Mutable. After a statement if this pointer points to a node it will recursively travel down random children until no children and will use the depth to output a byte |
| } | System Value Pointer | Regular pointer but can be modified by interacting with System Call Pointer |
| { | System Call Pointer | Mutable. After a statement if this pointer points to a node it will count a depth similar to the Output Pointer. It would then look at a list of syscall functions defined by the implementation of the language, If the function exists it would call it and use the possible depth of System Value Pointer as an argument and the function's return value would be used to make a possible chain referenced by System Value Pointer. |

## Pointer Expression

An expression for a node.

| Value | Type | Description |
| --- | --- | --- |
| `Pointer` | Pointer | Represents the possible node referenced by the pointer |
| `Pointer` + | Next Max | Represents a possible node with the highest weight of the children of the possible node referenced by the pointer |
| `Pointer` - | Next Min | Represents a possible node with the lowest weight of the children of the possible node referenced by the pointer |
| `Pointer` * `Integer` | Next Specific | Represents a possible node with the weight specified from the children of the possible node referenced by the pointer |
| `Pointer` $ | Next Random | Represents a random possible node from the children of the possible node referenced by the pointer |

## Weight Expression

An expression for a weight.

| Value | Type | Description |
| --- | --- | --- |
| `Pointer Expression` | Weight Of | Represents the weight of the node of the pointer expression |
| `Pointer Expression` ^ | Incremented Weight Of | Weight Of but incremented by 1 |
| `Pointer Expression` V | Incremented Weight Of | Weight Of but decremented by 1 |

## Statement

A unit that expresses an action.

Comments (anything surrounded by square brackets) and whitespaces can be placed between each statement.
Whitespaces can be placed between the units of a statement.

| Value | Type | Description |
| --- | --- | --- |
| < | Label | A location in the program that can be jumped to |
| # | Debug | Displays debug information |
| `Pointer Expression` % `Weight Expression` | Set Weight | Sets the weight of the expressed node on the left to the weight on the right |
| `Pointer Expression` > `Pointer Expression` | Connect Nodes | Adds a connection from the first node to the second |
| `Pointer Expression` / `Pointer Expression` | Disconenct Nodes | Opposite of Connect Nodes |
| `Pointer` = `Pointer Expression` | Set Pointer | Makes the pointer variable point to the node on the right
| @ `Integer` | Jump | Jumps forward or backward a specified amount of labels where 0 would be the current location and would be invalid |
| ? `Pointer Expression` : `Pointer Expression` @ `Integer` | Jump If Equal | Similar to jump but would only jump if the two pointer expressions resolve to the same nodes |
| ? `Pointer Expression` ; `Pointer Expression` @ `Integer` | Jump If Not Equal | Similar to Jump If Equal but inverted condition |
