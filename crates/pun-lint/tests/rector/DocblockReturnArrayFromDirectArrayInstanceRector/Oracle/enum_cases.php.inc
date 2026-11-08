<?php

enum Suit
{
    case Hearts;
    case Spades;
}

final class EnumCases
{
    public function run()
    {
        return [Suit::Hearts, 'a'];
    }
}
-----
<?php

enum Suit
{
    case Hearts;
    case Spades;
}

final class EnumCases
{
    /**
     * @return array<int, string|\Suit::Hearts>
     */
    public function run()
    {
        return [Suit::Hearts, 'a'];
    }
}
