<?php

class SomeClass
{
    public function go(): void
    {
        $this->run([1]);
    }

    /**
     * @param mixed[] $items-desc
     */
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
        $this->run([1]);
    }

    /**
     * @param int[] $items -desc
     */
    private function run(array $items): void
    {
    }
}
