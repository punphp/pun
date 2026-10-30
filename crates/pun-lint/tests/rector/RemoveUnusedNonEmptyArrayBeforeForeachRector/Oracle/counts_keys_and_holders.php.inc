<?php

namespace Oracle\ForeachGuardKeys;

trait Listing
{
    private array $rows = [[1]];

    public function fromTrait(): void
    {
        if ($this->rows[0] !== []) {
            foreach ($this->rows[0] as $row) {
                echo $row;
            }
        }
    }
}

final class Hooked
{
    public array $items {
        set(array $value) {
            if ($value) {
                foreach ($value as $item) {
                    echo $item;
                }
            }
            $this->items = $value;
        }
    }
}

final class Keys
{
    public function lists(): void
    {
        $lists = [[1], [2]];
        if ($lists[1] !== []) {
            foreach ($lists[1] as $item) {
                echo $item;
            }
        }

        if ($lists['x'] !== []) {
            foreach ($lists['x'] as $item) {
                echo $item;
            }
        }
    }

    public function variableKey(array $map, string $key): void
    {
        if ($map[$key] !== []) {
            foreach ($map[$key] as $item) {
                echo $item;
            }
        }
    }

    public function mixedArray($data): void
    {
        $data['key'] = [1];
        if (!empty($data['key'])) {
            foreach ($data['key'] as $item) {
                echo $item;
            }
        }
    }

    public const KEY = 'key';

    /** @param array<string, list<int>> $data */
    public function documentedArray($data, $other): void
    {
        $other['key'] = [1];
        echo 'start';
        strlen('x');
        $data[self::KEY] = [1];
        if (!empty($data['key'])) {
            foreach ($data['key'] as $item) {
                echo $item;
            }
        }
    }

    /** @param array<string, list<int>> $data */
    public function documentedStatements($data): void
    {
        echo 'start';
        strlen('x');
        $data['key'] = [1];
        if (!empty($data['key'])) {
            foreach ($data['key'] as $item) {
                echo $item;
            }
        }
    }

    public function counts(array $items, int $n): void
    {
        if (!$items) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if ($items === []) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if (count($items) > 2) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if ($n > 0) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if (count(value: $items) > 0) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if (array() !== $items) {
            foreach ($items as $item) {
                echo $item;
            }
        }
    }

    public function emptyElse(array $items): void
    {
        if (empty($items)) {
            echo 'none';
        } else {
            echo 'some';
        }
        foreach ($items as $item) {
            echo $item;
        }
    }

    public function notEmpty(array $items): void
    {
        if (isset($items[0])) {
            return;
        }
        foreach ($items as $item) {
            echo $item;
        }
    }

    public function otherEmpty(array $items, array $others): void
    {
        if (empty($others)) {
            return;
        }
        foreach ($items as $item) {
            echo $item;
        }
    }
}
-----
<?php

namespace Oracle\ForeachGuardKeys;

trait Listing
{
    private array $rows = [[1]];

    public function fromTrait(): void
    {
        if ($this->rows[0] !== []) {
            foreach ($this->rows[0] as $row) {
                echo $row;
            }
        }
    }
}

final class Hooked
{
    public array $items {
        set(array $value) {
            foreach ($value as $item) {
                echo $item;
            }
            $this->items = $value;
        }
    }
}

final class Keys
{
    public function lists(): void
    {
        $lists = [[1], [2]];
        foreach ($lists[1] as $item) {
            echo $item;
        }

        if ($lists['x'] !== []) {
            foreach ($lists['x'] as $item) {
                echo $item;
            }
        }
    }

    public function variableKey(array $map, string $key): void
    {
        if ($map[$key] !== []) {
            foreach ($map[$key] as $item) {
                echo $item;
            }
        }
    }

    public function mixedArray($data): void
    {
        $data['key'] = [1];
        if (!empty($data['key'])) {
            foreach ($data['key'] as $item) {
                echo $item;
            }
        }
    }

    public const KEY = 'key';

    /** @param array<string, list<int>> $data */
    public function documentedArray($data, $other): void
    {
        $other['key'] = [1];
        echo 'start';
        strlen('x');
        $data[self::KEY] = [1];
        foreach ($data['key'] as $item) {
            echo $item;
        }
    }

    /** @param array<string, list<int>> $data */
    public function documentedStatements($data): void
    {
        echo 'start';
        strlen('x');
        $data['key'] = [1];
        foreach ($data['key'] as $item) {
            echo $item;
        }
    }

    public function counts(array $items, int $n): void
    {
        if (!$items) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if ($items === []) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if (count($items) > 2) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        if ($n > 0) {
            foreach ($items as $item) {
                echo $item;
            }
        }

        foreach ($items as $item) {
            echo $item;
        }

        foreach ($items as $item) {
            echo $item;
        }
    }

    public function emptyElse(array $items): void
    {
        if (empty($items)) {
            echo 'none';
        } else {
            echo 'some';
        }
        foreach ($items as $item) {
            echo $item;
        }
    }

    public function notEmpty(array $items): void
    {
        if (isset($items[0])) {
            return;
        }
        foreach ($items as $item) {
            echo $item;
        }
    }

    public function otherEmpty(array $items, array $others): void
    {
        if (empty($others)) {
            return;
        }
        foreach ($items as $item) {
            echo $item;
        }
    }
}
