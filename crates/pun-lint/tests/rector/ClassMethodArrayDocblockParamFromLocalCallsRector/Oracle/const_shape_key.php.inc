<?php

class SomeClass
{
    public const A = 'a';

    /**
     * @param array{SomeClass::A: int} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public const A = 'a';

    /**
     * @param array{SomeClass::A: int} $x
     */
    public function go(array $x): void
    {
        $this->run($x);
    }

    /**
     * @param array<string, int> $items
     */
    private function run(array $items): void
    {
    }
}
