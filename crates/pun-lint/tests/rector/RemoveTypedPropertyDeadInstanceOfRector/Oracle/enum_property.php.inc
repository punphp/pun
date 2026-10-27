<?php

namespace App\EnumProperty;

enum Suit
{
    case Hearts;
}

final class Cards
{
    private Suit $suit;

    public function __construct()
    {
        $this->suit = Suit::Hearts;
    }

    public function check()
    {
        if ($this->suit instanceof Suit) {
            echo 1;
        }
    }
}
-----
<?php

namespace App\EnumProperty;

enum Suit
{
    case Hearts;
}

final class Cards
{
    private Suit $suit;

    public function __construct()
    {
        $this->suit = Suit::Hearts;
    }

    public function check()
    {
        echo 1;
    }
}
