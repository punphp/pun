<?php

trait Variants
{
    public function param(array $given): bool
    {
        return empty($given);
    }

    public function literal(): bool
    {
        $items = [];

        return empty($items);
    }

    public function fromParam(array $given): bool
    {
        $items = $given;

        return empty($items);
    }

    public function fromBuiltin(array $given): bool
    {
        $items = array_values($given);

        return empty($items);
    }
}
-----
<?php

trait Variants
{
    public function param(array $given): bool
    {
        return $given === [];
    }

    public function literal(): bool
    {
        $items = [];

        return $items === [];
    }

    public function fromParam(array $given): bool
    {
        $items = $given;

        return $items === [];
    }

    public function fromBuiltin(array $given): bool
    {
        $items = array_values($given);

        return $items === [];
    }
}
