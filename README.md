# Henkan!

Ergonomic unit converter in the terminal.

## Grammar

As of now, for the MVP, this is the entire grammar for the unit converter.

`<statemet> := <value> <operator> <unit>`
`<value> := <number><unit>`
`<operator> := to | in | as`
`<unit> := mm | cm | m | km | in | ft | MM | CM | M | KM | IN | FT | C | F`
`<number> := <integer> | <double>`
`<digit> := 0|1|2|3|4|5|6|7|8|9`
