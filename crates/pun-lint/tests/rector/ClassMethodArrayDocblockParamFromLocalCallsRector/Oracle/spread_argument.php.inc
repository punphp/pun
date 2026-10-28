<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(...[['a']]);
    }

    private function run(array $items): void
    {
    }
}
