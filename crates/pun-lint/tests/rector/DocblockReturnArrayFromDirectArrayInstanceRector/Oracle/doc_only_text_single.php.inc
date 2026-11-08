<?php

final class SomeClass
{
    /** Just text. */
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
     * Just text.
     * @return array<int, string>
     */
    public function run()
    {
        return ['a', 'b'];
    }
}
