<?php

namespace Rector\Tests\DeadCode\Rector\Array_\RemoveDuplicatedArrayKeyRector\Fixture;

class Fixture
{
    public function lists()
    {
        $items = [
            1 => 'A',
            1 => 'B',
            1 => 'A'
        ];
    }
}

?>
-----
<?php

namespace Rector\Tests\DeadCode\Rector\Array_\RemoveDuplicatedArrayKeyRector\Fixture;

class Fixture
{
    public function lists()
    {
        $items = [
            1 => 'A'
        ];
    }
}

?>
