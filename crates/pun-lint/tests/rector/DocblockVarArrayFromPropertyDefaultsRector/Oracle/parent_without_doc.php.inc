<?php

class ParentWithoutDoc
{
    public array $items = [];
}

class SomeClass extends ParentWithoutDoc
{
    public array $items = [1];
}
-----
<?php

class ParentWithoutDoc
{
    public array $items = [];
}

class SomeClass extends ParentWithoutDoc
{
    /**
     * @var int[]
     */
    public array $items = [1];
}
