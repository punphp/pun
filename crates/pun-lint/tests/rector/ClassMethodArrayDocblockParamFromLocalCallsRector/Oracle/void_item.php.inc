<?php

class SomeClass
{
    public function go(): void
    {
        $this->run([$this->noop()]);
    }

    private function noop(): void
    {
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
        $this->run([$this->noop()]);
    }

    private function noop(): void
    {
    }

    /**
     * @param null[] $items
     */
    private function run(array $items): void
    {
    }
}
