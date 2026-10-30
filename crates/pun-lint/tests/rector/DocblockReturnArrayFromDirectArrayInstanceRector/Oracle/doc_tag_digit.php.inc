<?php

final class SomeClass
{
    /**
     * @1abc text
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
     * @1abc text
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
