<?php

class SomeClass
{
    public function go(object $other): void
    {
        $this->run([$other]);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(object $other): void
    {
        $this->run([$other]);
    }

    /**
     * @param object[] $items
     */
    private function run(array $items): void
    {
    }
}
