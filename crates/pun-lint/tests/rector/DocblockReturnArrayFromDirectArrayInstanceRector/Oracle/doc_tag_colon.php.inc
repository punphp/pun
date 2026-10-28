<?php

final class SomeClass
{
    /**
     * @phpstan:pure
     */
    public function run()
    {
        return [1, 2];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @phpstan:pure
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
