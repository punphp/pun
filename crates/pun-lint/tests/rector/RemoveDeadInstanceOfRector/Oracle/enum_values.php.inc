<?php

namespace App\Enums;

enum Suit
{
    case Hearts;
}

final class Cards
{
    public function check(Suit $suit)
    {
        if ($suit instanceof Suit) {
            echo 1;
        }
    }
}
-----
<?php

namespace App\Enums;

enum Suit
{
    case Hearts;
}

final class Cards
{
    public function check(Suit $suit)
    {
        echo 1;
    }
}
