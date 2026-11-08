<?php

class SomeClass
{
    public function go(): void
    {
        $this->run($this);
    }

    private function run(array $items): void
    {
    }
}
-----
<?php

class SomeClass
{
    public function go(): void
    {
        $this->run($this);
    }

    /**
     * @param \SomeClass $items
     */
    private function run(array $items): void
    {
    }
}
