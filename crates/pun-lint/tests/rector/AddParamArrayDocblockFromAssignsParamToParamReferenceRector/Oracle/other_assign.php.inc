<?php

final class OtherAssign
{
    public function run(array &$items)
    {
        $other = 1;
        $items[] = $other;
    }
}
-----
<?php

final class OtherAssign
{
    /**
     * @param int[] $items
     */
    public function run(array &$items)
    {
        $other = 1;
        $items[] = $other;
    }
}
