<?php

class A
{
    public function a(array $items)
    {
        $ok = true;
        // check every item
        foreach ($items as $item) {
            if (! $item->valid()) {
                $ok = false;
                break;
            }
        }

        return $ok;
    }

    public function b(array $items)
    {
        foreach ($items as $k => $item):
            if ($item > $k):
                return false;
            endif;
        endforeach;
        return true;
    }

    public function c($items)
    {
        foreach ($items as $item) {
            if ($item) {
                return false;
            }
        }
        return true;
    }

    public function d()
    {
        $items = [1, 2];
        foreach ($items as $item) {
            if ($item === 1 || $item === 2) {
                return false;
            }
        }
        return true;
    }
}
-----
<?php

class A
{
    public function a(array $items)
    {
        $ok = array_all($items, fn($item) => $item->valid());
        return $ok;
    }

    public function b(array $items)
    {
        return array_all($items, fn($item, $k) => !($item > $k));
    }

    public function c($items)
    {
        foreach ($items as $item) {
            if ($item) {
                return false;
            }
        }
        return true;
    }

    public function d()
    {
        $items = [1, 2];
        return array_all($items, fn($item) => !($item === 1 || $item === 2));
    }
}
