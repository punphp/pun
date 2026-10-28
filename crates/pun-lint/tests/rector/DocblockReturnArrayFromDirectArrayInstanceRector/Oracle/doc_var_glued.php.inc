<?php

final class SomeClass
{
    /**
     * @var int(x)
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
     * @var int(x)
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
