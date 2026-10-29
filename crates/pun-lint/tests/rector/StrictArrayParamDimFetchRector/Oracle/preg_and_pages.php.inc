<?php

namespace Probe;

/**
 * @template TValue
 */
class Pages
{
    /**
     * @param  array<int, TValue>  $items
     */
    public function __construct(public array $items)
    {
    }

    /**
     * @param  callable(TValue, int): mixed  $callback
     * @return $this
     */
    public function through(callable $callback)
    {
        return $this;
    }
}

function mentions(string $value)
{
    return preg_replace_callback('/<u:(\d+)>/', function ($matches) {
        return '@'.$matches[1];
    }, $value);
}

function words(string $value)
{
    return preg_replace_callback('/\w+/', function ($matches) {
        return $matches[0];
    }, $value);
}

function pages()
{
    $p = new Pages([['id' => 4], ['id' => 5]]);

    $p->through(function ($item) {
        $item['id'] = $item['id'] + 2;

        return $item;
    });
}
-----
<?php

namespace Probe;

/**
 * @template TValue
 */
class Pages
{
    /**
     * @param  array<int, TValue>  $items
     */
    public function __construct(public array $items)
    {
    }

    /**
     * @param  callable(TValue, int): mixed  $callback
     * @return $this
     */
    public function through(callable $callback)
    {
        return $this;
    }
}

function mentions(string $value)
{
    return preg_replace_callback('/<u:(\d+)>/', function ($matches) {
        return '@'.$matches[1];
    }, $value);
}

function words(string $value)
{
    return preg_replace_callback('/\w+/', function (array $matches) {
        return $matches[0];
    }, $value);
}

function pages()
{
    $p = new Pages([['id' => 4], ['id' => 5]]);

    $p->through(function (array $item) {
        $item['id'] = $item['id'] + 2;

        return $item;
    });
}
