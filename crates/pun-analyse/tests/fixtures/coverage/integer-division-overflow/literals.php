<?php
namespace IntegerDivisionOverflow;

function minimumDivision(): void
{
    (-9223372036854775807 - 1) / -1; // result
}

function minimumModulo(): void
{
    (-9223372036854775807 - 1) % -1; // result
}

function negativeNeighbor(): void
{
    -9223372036854775807 / -1; // result
}

function minimumPositiveDivisor(): void
{
    (-9223372036854775807 - 1) / 1; // result
}

function minimumHalving(): void
{
    (-9223372036854775807 - 1) / -2; // result
}

function positiveNeighbor(): void
{
    9223372036854775807 / -1; // result
}

function positiveDivision(): void
{
    9223372036854775807 / 1; // result
}

function integral(): void
{
    6 / 3; // result
}

function fractional(): void
{
    7.0 / 2; // result
}

function negativeFractional(): void
{
    -7.0 / 2; // result
}

function negativeModulo(): void
{
    -7 % 3; // result
}

function neighborModulo(): void
{
    -9223372036854775807 % -1; // result
}

function divisionByZero(): void
{
    1 / 0; // result
}

function moduloByZero(): void
{
    1 % 0; // result
}

function overflowingLiteralSpelling(): void
{
    -9223372036854775808 / -1; // result
}
