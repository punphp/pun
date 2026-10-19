<?php

use Pun\Names\Lib\Item;

class GlobalThing
{
}

final class GlobalMaker
{
    public function item()
    {
        return new Item();
    }

    public function thing()
    {
        return new GlobalThing();
    }
}
-----
<?php

use Pun\Names\Lib\Item;

class GlobalThing
{
}

final class GlobalMaker
{
    public function item(): \Pun\Names\Lib\Item
    {
        return new Item();
    }

    public function thing(): \GlobalThing
    {
        return new GlobalThing();
    }
}
