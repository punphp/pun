<?php

final class SomeClass
{
    /**
     * Some text.
     *
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
     * Some text.
     *
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
