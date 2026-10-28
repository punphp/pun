<?php

final class SomeClass
{
    /**
     * @_internal yes
     * @\Foo\Bar(1)
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
     * @_internal yes
     * @\Foo\Bar(1)
     * @return array<int, int>
     */
    public function run()
    {
        return [1, 2];
    }
}
