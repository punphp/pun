<?php

final class Item
{
}

final class LoopWritesAndCalls
{
    /**
     * @return list<Item>
     */
    private function items()
    {
        return [];
    }

    public function keyed(): array
    {
        $byKey = [];

        foreach ($this->items() as $key => $item) {
            if (empty($byKey)) {
                $byKey['first'] = $item;
            }
        }

        return $byKey;
    }

    public function otherWrites(array $values, \stdClass $o): bool
    {
        foreach ($values as $value) {
            $o->last = $value;
            if (!empty($values)) {
                return true;
            }
        }

        return false;
    }

    public function call(array $values): bool
    {
        return empty(array_filter($values));
    }
}

trait FromThis
{
    /** @var array */
    private $list = [];

    public function copy(): bool
    {
        $copy = $this->list;

        return empty($copy);
    }
}
-----
<?php

final class Item
{
}

final class LoopWritesAndCalls
{
    /**
     * @return list<Item>
     */
    private function items()
    {
        return [];
    }

    public function keyed(): array
    {
        $byKey = [];

        foreach ($this->items() as $key => $item) {
            if (empty($byKey)) {
                $byKey['first'] = $item;
            }
        }

        return $byKey;
    }

    public function otherWrites(array $values, \stdClass $o): bool
    {
        foreach ($values as $value) {
            $o->last = $value;
            if ($values !== []) {
                return true;
            }
        }

        return false;
    }

    public function call(array $values): bool
    {
        return empty(array_filter($values));
    }
}

trait FromThis
{
    /** @var array */
    private $list = [];

    public function copy(): bool
    {
        $copy = $this->list;

        return empty($copy);
    }
}
