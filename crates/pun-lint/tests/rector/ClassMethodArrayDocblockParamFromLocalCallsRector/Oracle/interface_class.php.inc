<?php

interface SomeInterface
{
    public function run(array $items): void;
}

class SomeClass
{
    public function go(SomeInterface $other): void
    {
        $other->run(['a']);
        $this->run(['a']);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

interface SomeInterface
{
    public function run(array $items): void;
}

class SomeClass
{
    public function go(SomeInterface $other): void
    {
        $other->run(['a']);
        $this->run(['a']);
    }

    /**
     * @param string[] $items
     */
    private function run(array $items): void
    {
    }
}
