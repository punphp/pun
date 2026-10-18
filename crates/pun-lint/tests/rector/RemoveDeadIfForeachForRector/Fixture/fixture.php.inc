<?php

namespace Rector\Tests\DeadCode\Rector\For_\RemoveDeadIfForeachForRector\Fixture;

class Fixture
{
    public function run($someObject, $differentValue)
    {
        $value = 5;
        if ($value) {
        }

        if ($someObject->run()) {
        }

        $values = ['a', 'b', 'c'];
        foreach ($values as $value) {
        }

        return $differentValue;
    }

    public function forMe()
    {
        for ($i = 0; $i < 5; ++$i) {
        }
    }
}

?>
-----
<?php

namespace Rector\Tests\DeadCode\Rector\For_\RemoveDeadIfForeachForRector\Fixture;

class Fixture
{
    public function run($someObject, $differentValue)
    {
        $value = 5;

        if ($someObject->run()) {
        }

        $values = ['a', 'b', 'c'];

        return $differentValue;
    }

    public function forMe()
    {
    }
}

?>
