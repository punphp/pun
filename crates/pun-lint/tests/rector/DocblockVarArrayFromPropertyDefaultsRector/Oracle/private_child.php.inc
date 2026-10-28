<?php

class ParentDoc
{
    /** @var int[] */
    public array $items = [];
}

class SomeClass extends ParentDoc
{
    private array $other = [1];
}
-----
<?php

class ParentDoc
{
    /** @var int[] */
    public array $items = [];
}

class SomeClass extends ParentDoc
{
    /**
     * @var int[]
     */
    private array $other = [1];
}
