<?php

final class SomeClass
{
    /**
     * @return array
     * @return int
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @return array<int, string>
     * @return int
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
