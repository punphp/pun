<?php

class Probe
{
    /**
     * @param  callable  $callback
     */
    protected static function normalizeMethod($callback)
    {
        $class = is_string($callback[0]) ? $callback[0] : get_class($callback[0]);

        return "{$class}@{$callback[1]}";
    }

    public function echos()
    {
        $callback = function ($matches) {
            $whitespace = empty($matches[3]) ? '' : $matches[3].$matches[3];

            return $matches[1] ? substr($matches[0], 1) : $whitespace.$matches[2];
        };

        return $callback;
    }

    public function through($p)
    {
        $p->through(function ($item) {
            $item['id'] = $item['id'] + 2;

            return $item;
        });
    }

    public function plain($item)
    {
        return $item['id'] + 2;
    }
}
-----
<?php

class Probe
{
    /**
     * @param  callable  $callback
     */
    protected static function normalizeMethod($callback)
    {
        $class = is_string($callback[0]) ? $callback[0] : get_class($callback[0]);

        return "{$class}@{$callback[1]}";
    }

    public function echos()
    {
        $callback = function ($matches) {
            $whitespace = empty($matches[3]) ? '' : $matches[3].$matches[3];

            return $matches[1] ? substr($matches[0], 1) : $whitespace.$matches[2];
        };

        return $callback;
    }

    public function through($p)
    {
        $p->through(function (array $item) {
            $item['id'] = $item['id'] + 2;

            return $item;
        });
    }

    public function plain(array $item)
    {
        return $item['id'] + 2;
    }
}
