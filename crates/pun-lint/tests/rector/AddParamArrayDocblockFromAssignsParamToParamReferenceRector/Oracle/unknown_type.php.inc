<?php

final class UnknownType
{
    public function run(array &$items)
    {
        $items[] = unknown_function();
    }
}
-----
<?php

final class UnknownType
{
    /**
     * @param mixed[] $items
     */
    public function run(array &$items)
    {
        $items[] = unknown_function();
    }
}
