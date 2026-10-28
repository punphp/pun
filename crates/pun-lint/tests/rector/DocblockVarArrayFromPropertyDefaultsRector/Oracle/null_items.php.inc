<?php

class SomeClass
{
    private array $items = [null, null];
}
-----
<?php

class SomeClass
{
    /**
     * @var null[]
     */
    private array $items = [null, null];
}
