<?php

final class MultilineDoc
{
    /**
     * @param array<
     *     int
     * > $other
     */
    public function run(array &$items, $other)
    {
        $items[] = 1;
    }
}
-----
<?php

final class MultilineDoc
{
    /**
     * @param array<
     *     int
     * > $other
     * @param int[] $items
     */
    public function run(array &$items, $other)
    {
        $items[] = 1;
    }
}
