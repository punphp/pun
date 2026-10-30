<?php

class SomeClass
{
    private array $items = [9999999999999999999];
}
-----
<?php

class SomeClass
{
    /**
     * @var 1.0E+19[]
     */
    private array $items = [9999999999999999999];
}
