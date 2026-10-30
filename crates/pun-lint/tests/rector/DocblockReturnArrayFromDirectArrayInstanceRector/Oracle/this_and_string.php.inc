<?php

final class SomeClass
{
    public function run()
    {
        return [$this, 'a'];
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @return array<int, $this|string>
     */
    public function run()
    {
        return [$this, 'a'];
    }
}
