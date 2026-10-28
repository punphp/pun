<?php

final class SomeClass
{
    /**
     * @see Foo::bar()
     * @throws \Exception when it fails
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
     * @see Foo::bar()
     * @throws \Exception when it fails
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
