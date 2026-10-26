<?php

trait FromThisLocal
{
    protected function items(): array
    {
        return [];
    }

    public function direct(): bool
    {
        $items = $this->items();

        return empty($items);
    }

    public function closureUse(): \Closure
    {
        $items = $this->items();

        return function () use ($items) {
            return ! empty($items);
        };
    }

    public function local(array $given): bool
    {
        $items = $given;

        return empty($items);
    }
}
-----
<?php

trait FromThisLocal
{
    protected function items(): array
    {
        return [];
    }

    public function direct(): bool
    {
        $items = $this->items();

        return empty($items);
    }

    public function closureUse(): \Closure
    {
        $items = $this->items();

        return function () use ($items) {
            return ! empty($items);
        };
    }

    public function local(array $given): bool
    {
        $items = $given;

        return $items === [];
    }
}
