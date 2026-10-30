<?php

final class ClosureAssign
{
    public function run(array &$items)
    {
        $f = function () use (&$items) {
            $items[] = 1;
        };
        $o = new class ($items[] = 2) {
            public function __construct($x)
            {
            }
        };
    }
}
-----
<?php

final class ClosureAssign
{
    /**
     * @param int[] $items
     */
    public function run(array &$items)
    {
        $f = function () use (&$items) {
            $items[] = 1;
        };
        $o = new class ($items[] = 2) {
            public function __construct($x)
            {
            }
        };
    }
}
