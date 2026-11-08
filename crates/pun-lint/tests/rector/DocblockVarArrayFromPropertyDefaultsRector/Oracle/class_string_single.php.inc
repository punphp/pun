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
     * @var array<int, class-string<\SomeClass>>
     */
    private array $items = [SomeClass::class];
}
