<?php

final class ExistingText
{
    /**
     * Fills the items.
     */
    public function run(array &$items)
    {
        $items[] = 42;
    }
}
-----
<?php

final class ExistingText
{
    /**
     * Fills the items.
     * @param int[] $items
     */
    public function run(array &$items)
    {
        $items[] = 42;
    }
}
