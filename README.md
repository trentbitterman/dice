# Dice

## What is dice?

Dice is a program that can "roll" any number of dice with any positive and
non-zero number of sides.

## Usage

At the moment, it only takes two arguments: number of dice and number of sides.
These can be specified using the ```-n``` and ```-s``` options.

### Example

```text
$ dice -n 11 -s 5
5 5 4 1 5 3 4 4 2 2 1
```

## Future Goals

There are a few features I am currently planning on adding in the near future
and probably more after that. They are:

* Output rolls to a file
* Get statistical information about rolls, e.g. min, max, mean, sum, mode
* Read a file specifying rolls as input in the format: "2d6 1d20", for example.
