<?php

class ParentPrivate
{
    /** @var int[] */
    private array $items = [];
}

class SomeClass extends ParentPrivate
{
    public array $items = [1];
}
-----
<?php

class ParentPrivate
{
    /** @var int[] */
    private array $items = [];
}

class SomeClass extends ParentPrivate
{
    /**
     * @var int[]
     */
    public array $items = [1];
}
