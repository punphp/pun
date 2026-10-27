<?php

namespace N;

class C
{
    public function run()
    {
        $a = collection([]);
        $b = \COLLECTION([1, 2], 3);
        $c = collection([])->map($f);
        $d = collection(
            // items
            [1, 2],
            $b
        );
        $e = collection(...$x);
        $f = collection(items: [1]);
    }
}
-----
<?php

namespace N;

class C
{
    public function run()
    {
        $a = new \Collection([]);
        $b = new \Collection([1, 2], 3);
        $c = (new \Collection([]))->map($f);
        $d = new \Collection(
            // items
            [1, 2],
            $b
        );
        $e = new \Collection(...$x);
        $f = new \Collection(items: [1]);
    }
}
