<?php

class SomeClass
{
    public function go(): void
    {
        $self = $this;
        $self->run([$self]);
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
        $self = $this;
        $self->run([$self]);
    }

    /**
     * @param $this[] $items
     */
    private function run(array $items): void
    {
    }
}
