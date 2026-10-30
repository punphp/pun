<?php

final class DimAssign
{
    public function run(array &$items)
    {
        $items['a'] = 'value';
    }
}
-----
<?php

final class DimAssign
{
    /**
     * @param string[] $items
     */
    public function run(array &$items)
    {
        $items['a'] = 'value';
    }
}
