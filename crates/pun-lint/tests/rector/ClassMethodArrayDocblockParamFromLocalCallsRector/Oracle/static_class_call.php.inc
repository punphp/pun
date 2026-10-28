<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        SomeClass::run(['a']);
    }

    private static function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        SomeClass::run(['a']);
    }

    /**
     * @param string[] $items
     */
    private static function run(array $items): void
    {
    }
}
