<?php

namespace Overriding;

interface Zips
{
    /**
     * @param iterable<int> ...$items
     */
    public function zip($items);
}

class Zipper implements Zips
{
    /**
     * @param iterable<int> ...$items
     */
    public function zip($items)
    {
        return func_get_args();
    }
}

class Icons
{
    protected static string|\BackedEnum|null $icon = null;

    protected int $count = 0;
}

class Page extends Icons
{
    protected static \BackedEnum|string|null $icon = 'lock';

    protected string $count = '';
}
