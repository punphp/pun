<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $mixed = $this->mixed();
        $mixed->run(['a']);
    }

    private function run(array $items): void
    {
    }

    public function mixed()
    {
    }
}
