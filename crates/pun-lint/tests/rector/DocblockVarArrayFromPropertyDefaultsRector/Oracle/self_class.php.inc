<?php

class SomeClass
{
    private array $items = [self::class];
}
-----
<?php

class SomeClass
{
    /**
     * @var class-string<\SomeClass>[]
     */
    private array $items = [self::class];
}
