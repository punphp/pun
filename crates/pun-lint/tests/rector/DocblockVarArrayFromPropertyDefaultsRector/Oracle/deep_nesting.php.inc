<?php

class SomeClass
{
    private array $items = [[[['a']]]];
}
-----
<?php

class SomeClass
{
    /**
     * @var array<int, array<int, array<int, string[]>>>
     */
    private array $items = [[[['a']]]];
}
