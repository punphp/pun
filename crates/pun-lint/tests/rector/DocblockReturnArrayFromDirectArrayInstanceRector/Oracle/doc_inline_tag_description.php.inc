<?php

final class SomeClass
{
    /**
     * @return array Some {@link Foo} text
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
     * @return array<int, string> Some {@link Foo} text
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
