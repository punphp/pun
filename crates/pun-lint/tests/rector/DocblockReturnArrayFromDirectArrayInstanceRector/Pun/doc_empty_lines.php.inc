<?php

final class SomeClass
{
    /**
     *
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
     *
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
