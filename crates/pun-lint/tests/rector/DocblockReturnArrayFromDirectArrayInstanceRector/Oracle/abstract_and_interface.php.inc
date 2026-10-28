<?php

abstract class AbstractAndInterface
{
    abstract public function run(): array;

    public function other(): array
    {
        return [1, 2];
    }
}
-----
<?php

abstract class AbstractAndInterface
{
    abstract public function run(): array;

    /**
     * @return array<int, int>
     */
    public function other(): array
    {
        return [1, 2];
    }
}
