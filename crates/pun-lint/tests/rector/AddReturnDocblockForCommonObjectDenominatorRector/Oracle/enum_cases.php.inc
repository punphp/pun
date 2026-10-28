<?php

enum Suit
{
    case Hearts;
    case Spades;
}

final class SomeClass
{
    public function run(): array
    {
        return [Suit::Hearts, Suit::Spades];
    }
}
-----
<?php

enum Suit
{
    case Hearts;
    case Spades;
}

final class SomeClass
{
    /**
     * @return \Suit[]
     */
    public function run(): array
    {
        return [Suit::Hearts, Suit::Spades];
    }
}
