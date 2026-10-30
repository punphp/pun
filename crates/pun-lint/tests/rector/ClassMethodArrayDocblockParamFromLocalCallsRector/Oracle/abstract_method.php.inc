<?php

abstract class SomeClass
{
    public function go(): void
    {
        $this->run(['a']);
    }

    abstract protected function run(array $items): void;
}
-----
<?php

abstract class SomeClass
{
    public function go(): void
    {
        $this->run(['a']);
    }

    /**
     * @param string[] $items
     */
    abstract protected function run(array $items): void;
}
