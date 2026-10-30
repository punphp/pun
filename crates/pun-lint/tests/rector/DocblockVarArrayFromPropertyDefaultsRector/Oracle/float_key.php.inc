<?php

class SomeClass
{
    private array $items = [1.5 => 'a'];
}
-----
<?php

class SomeClass
{
    /**
     * @var string[]
     */
    private array $items = [1.5 => 'a'];
}
