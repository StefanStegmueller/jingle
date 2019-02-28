# jinglepi

An experimental project to play jingles with a Raspberry Pi and to fiddle with the rust language.
The program generates audio signals using the Raspberry Pi 3 Modell B+ hardware.

## Build
```
cargo build
```

## Musical notation

Jingles are noted in rather primitive way.
Frequencies in are mapped to musical notes (e.g note:C4 -> frequency: 261.63).
The duration of a signal is noted in milliseconds.
Rows of a csv file are looped to generate frequencies for a specific duration.

### Example

| Note | Duration | Row in CSV file |
| ---- | :------: | :-------------: |
| C4   |   100    |     C4,100      |
| E4   |   150    |     E4,150      |

Example files:

- [mario jingle](jingles/mario)
- [zelda jingle](jingles/zelda)

## Audio Output

In digital mode the audio signals are simple rectangle waves emitted from a gpio pin of the pi. The duty cycle is customizable.
In Analog mode a MCP4725 12-Bit DAC breakout board gets controlled via i2c-bus. Waveforms for analog output are square, sine, triangle and sawtooth.

## Findings during implementation

Due the multitasking nature of the raspbian os for the Raspberry Pi precize frequency generation is not possible. The precision in digital output mode seems sufficient. The sample rate for Analog output mode via i2c on the other hand is way too slow to generate acceptable analog signals. This seemed pretty obvious with a little calculation beforehand but I had hopes. Afterall it was a fun little project and maybe this helps someone in some way.

## Command Line Usage

```
USAGE:
    jinglepi [OPTIONS] <JINGLEFILE> <MODE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --duty <duty>                Sets the duty cycle for digital output [default: 50]
    -g, --gpio <gpio>                Sets the gpio pin for digital output [default: 2]
    -i, --i2caddress <i2caddress>    Sets the i2c address (hex) for analog output with dac [default: 62]
    -w, --wave <wave>                Sets the waveform for analog output [default: Square]  [possible values:
                                     Square, Sine, Triangle, Saw]

ARGS:
    <JINGLEFILE>    Sets the jingle file to use
    <MODE>          Set output mode [possible values: Digital, Analog]

EXAMPLE:
    jinglepi -g 2 -d 60 ../jingelfile Digital
```
