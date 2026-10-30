<?php

class SomeClass
{
    public function go(self $other): void
    {
        $this->run([$other::class]);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(self $other): void
    {
        $this->run([$other::class]);
    }

    /**
     * @param class-string<\SomeClass>[] $items
     */
    private function run(array $items): void
    {
    }
}
