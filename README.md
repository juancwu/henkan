# Henkan!

Ergonomic unit converter in the terminal.

## Grammar

As of now, for the MVP, this is the entire grammar for the unit converter.

`<command> := \<keyword> <command'>`
`<command'> := <arg> <command'> | E`
`<arg> := -<letter><arg'>`
`<arg'> := <letter><arg'> | E`
`<exp> := <value> <operator> <unit>`
`<value> := <metric> | <temperature>`
`<metric> := <number><metric_unit>`
`<temperature> := <number><temperature_unit>`
`<unit> := <metric_unit> | <temperature_unit>`
`<temperature_unit> := C | Celsius | F | Fahrenheit`
`<operator> := to | in | as`
`<metric_unit> := mm | cm | m | km`
`<number> := <integer> | <float>`
`<digit> := 0|1|2|3|4|5|6|7|8|9`
`<letter> := a-zA-Z`

## Keywords

This is a comprehensive list of keywords that are basically commands in our grammar.

-   `cp [-offset n]` copies the last result by default to the clipboard or specified the offset that needs to be copied.
-   `help` shows the help menu.

## Operation Process

Henkan basically works like a programming language intepreter. It will boot up an interface for the user to type,
then it will try to parse the line and evaluate it, and finally show the result back in the terminal.

This is basically an intepreter like NodeJS shell or Python's too.
