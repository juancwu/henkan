# Henkan!

Ergonomic calculator with a unit converter in the terminal.

## Grammar

Extensive grammar to make it an ergonomic calculator with a touch of unit conversion.
The calculator works line by line so there is no explicit need for a separator. However,
to make the grammar better to parse, we'll use an optional ';' as an indicator of end of line.

```
// Command related grammar
<command> := \<keyword> <command'>
<command'> := <arg> <command'> | E
<arg> := --<identifier><arg'>
<arg'> := <identifier> | <number> | E

// Calculator related grammar
<statement> := <expression> | <command> | <assignment>
<assignment> := <identifier> = <exp>
<identifier> := <letter><identifier'>
<identifier'> := <letter><identifier'> | E
<expression> := <value> <operator> <unit> | (<expression>)
<value> := <metric> | <temperature>
<metric> := <number><metric_unit>
<temperature> := <number><temperature_unit>
<unit> := <metric_unit> | <temperature_unit>
<temperature_unit> := C | Celsius | F | Fahrenheit
<operator> := to | in | as
<metric_unit> := pm | nm | um | mm | cm | dm | m | dam | hm | km | Mm | Gm | Tm
<number> := <integer> | <float>
<digit> := 0..9
<letter> := a..z | A..Z
```

## Keywords

This is a comprehensive list of keywords that are basically commands in our grammar.

-   `cp [-offset n]` copies the last result by default to the clipboard or specified the offset that needs to be copied.
-   `help` shows the help menu.

## Operation Process

Henkan basically works like a programming language intepreter. It will boot up an interface for the user to type,
then it will try to parse the line and evaluate it, and finally show the result back in the terminal.

This is basically an intepreter like NodeJS shell or Python's too.
