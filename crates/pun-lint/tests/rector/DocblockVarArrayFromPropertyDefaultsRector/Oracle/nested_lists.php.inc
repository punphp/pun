<?php

class SomeClass
{
    private array $items = [[1, 2], [3, 4]];
}
-----
<?php

class SomeClass
{
    /**
     * @var int[][]
     */
    private array $items = [[1, 2], [3, 4]];
}
