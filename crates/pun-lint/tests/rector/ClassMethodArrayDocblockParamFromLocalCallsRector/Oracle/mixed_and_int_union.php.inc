<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run([1]);
        $this->run($this->mixed());
    }

    private function run(array $items): void
    {
    }

    public function mixed()
    {
    }
}
