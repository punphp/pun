<?php

class SomeClass extends UnknownParent
{
    public array $items = [1];
}
-----
<?php

class SomeClass extends UnknownParent
{
    /**
     * @var int[]
     */
    public array $items = [1];
}
