<?php

enum Suit
{
    case Hearts;
    case Spades;
}

function pick(bool $red)
{
    if ($red) {
        return Suit::Hearts;
    }
    return Suit::Spades;
}

function maybe(bool $red)
{
    if ($red) {
        return 1;
    }
    return 2;
}
