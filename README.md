# Henkan!

Ergonomic unit converter in the terminal.

## Grammar

As of now, for the MVP, this is the entire grammar for the unit converter.

`<command> := <keyword> <command'>`
`<command'> := <arg> <command'> | E`
`<arg> := -<letter><arg'>`
`<arg'> := <letter><arg'> | E`
`<exp> := <value> <operator> <unit>`
`<value> := <number><unit>`
`<operator> := to | in | as`
`<unit> := mm | cm | m | km | in | ft | MM | CM | M | KM | IN | FT | C | F`
`<number> := <integer> | <float>`
`<digit> := 0|1|2|3|4|5|6|7|8|9`
`<letter> := a-zA-Z`

## Keywords

This is a comprehensive list of keywords that are basically commands in our grammar.

- `cp [-offset n]` copies the last result by default to the clipboard or specified the offset that needs to be copied.
- `help` shows the help menu.
