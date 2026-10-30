<?php

final class SomeClass
{
    public function run(array &$items)
    {
        $items[] = STDIN;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param resource[] $items
     */
    public function run(array &$items)
    {
        $items[] = STDIN;
    }
}
