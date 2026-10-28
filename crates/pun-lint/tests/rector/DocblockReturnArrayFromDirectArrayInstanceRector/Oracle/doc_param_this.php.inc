<?php

final class SomeClass
{
    /**
     * @param int $this
     */
    public function run($x = null)
    {
        return ['a', 'b'];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param int $this
     * @return array<int, string>
     */
    public function run($x = null)
    {
        return ['a', 'b'];
    }
}
