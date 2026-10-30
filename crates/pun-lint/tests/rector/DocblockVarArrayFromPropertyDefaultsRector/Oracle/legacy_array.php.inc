<?php

class SomeClass
{
    private array $items = array(1, 2);
}
-----
<?php

class SomeClass
{
    /**
     * @var int[]
     */
    private array $items = array(1, 2);
}
