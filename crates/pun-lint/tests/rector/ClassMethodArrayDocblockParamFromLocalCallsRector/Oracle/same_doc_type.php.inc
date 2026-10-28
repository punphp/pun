<?php

class SomeClass
{
    public function go(self $other, string $name): void
    {
        $this->run(['a']);
    }

    /**
     * @param string[]|null $items
     */
    private function run(?array $items): void
    {
    }
}
