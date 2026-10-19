<?php

class NoConstructor
{
}

class GlobalClass
{
    public function __construct(public int $public, private $untyped, private $removed)
    {
        echo $untyped;
    }
}
-----
<?php

class NoConstructor
{
}

class GlobalClass
{
    public function __construct(public int $public, $untyped)
    {
        echo $untyped;
    }
}
