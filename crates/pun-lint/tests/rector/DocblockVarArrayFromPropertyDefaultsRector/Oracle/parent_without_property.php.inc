<?php

class ParentWithout
{
}

class SomeClass extends ParentWithout
{
    public array $items = [1];
}
-----
<?php

class ParentWithout
{
}

class SomeClass extends ParentWithout
{
    /**
     * @var int[]
     */
    public array $items = [1];
}
