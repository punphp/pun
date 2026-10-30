<?php

final class SomeClass
{
    /**
     * @return mixed[]
     *
     * Some text.
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
     * @return array<int, int>
     *
     * Some text.
     */
    public function run()
    {
        return [1, 2];
    }
}
