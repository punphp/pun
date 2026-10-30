<?php

class SomeClass
{
    private array $items = [SomeClass::class];
}
-----
<?php

class SomeClass
{
    /**
     * @var class-string<\SomeClass>[]
     */
    private array $items = [SomeClass::class];
}
