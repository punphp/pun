<?php

class SomeClass
{
    private array $items = [true, false];
}
-----
<?php

class SomeClass
{
    /**
     * @var bool[]
     */
    private array $items = [true, false];
}
