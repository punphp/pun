<?php

namespace Oracle\ForeachGuard;

final class Holder
{
    private array $items = [];

    public function properties(): void
    {
        if ($this->items) {
            // keep this note
            foreach ($this->items as $item) {
                echo $item;
            }
        }
    }

    public function conditions(array $values, array $map, $mixed): void
    {
        if ([] !== $values) {
            foreach ($values as $value) {
                echo $value;
            }
        }

        if (count($values) >= 1) {
            foreach ($values as $value) {
                echo $value;
            }
        }

        if (1 <= count($values)) {
            foreach ($values as $value) {
                echo $value;
            }
        }

        if ($values && count($values) > 0) {
            foreach ($values as $value) {
                echo $value;
            }
        }

        if (!empty($mixed)) {
            foreach ($mixed as $value) {
                echo $value;
            }
        }

        if ($values != []) {
            foreach ($values as $value) {
                echo $value;
            }
        } else {
            echo 'none';
        }
    }

    public function emptyBefore(array $values): void
    {
        echo 'start';
        if (empty($values)) {
            return;
        }

        foreach ($values as $value) {
            echo $value;
        }
    }
}
-----
<?php

namespace Oracle\ForeachGuard;

final class Holder
{
    private array $items = [];

    public function properties(): void
    {
        // keep this note
        foreach ($this->items as $item) {
            echo $item;
        }
    }

    public function conditions(array $values, array $map, $mixed): void
    {
        foreach ($values as $value) {
            echo $value;
        }

        foreach ($values as $value) {
            echo $value;
        }

        foreach ($values as $value) {
            echo $value;
        }

        foreach ($values as $value) {
            echo $value;
        }

        if (!empty($mixed)) {
            foreach ($mixed as $value) {
                echo $value;
            }
        }

        if ($values != []) {
            foreach ($values as $value) {
                echo $value;
            }
        } else {
            echo 'none';
        }
    }

    public function emptyBefore(array $values): void
    {
        echo 'start';

        foreach ($values as $value) {
            echo $value;
        }
    }
}
