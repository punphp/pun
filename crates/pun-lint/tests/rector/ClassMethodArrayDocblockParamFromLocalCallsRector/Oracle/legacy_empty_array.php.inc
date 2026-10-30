<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(array());
        $this->run(['b']);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(array());
        $this->run(['b']);
    }

    /**
     * @param string[] $items
     */
    private function run(array $items): void
    {
    }
}
