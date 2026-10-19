<?php

namespace App;

final class M
{
    private $event;
    private $store;

    public function a($command)
    {
        return str_replace(['a'], ['b'], $command);
    }

    public function b()
    {
        return get_class($this->event);
    }

    public function c($name)
    {
        $this->store->forever($name, $id = str_replace('.', '', uniqid('', true)));

        return $id;
    }

    public function d($x)
    {
        return preg_replace('#a#', '', $x);
    }

    public function e(string $x)
    {
        return str_replace('a', 'b', $x);
    }

    public function f($name)
    {
        $id = str_replace('.', '', uniqid('', true));

        return $id;
    }

    public function g($o)
    {
        return get_class($o);
    }
}
-----
<?php

namespace App;

final class M
{
    private $event;
    private $store;

    public function a($command)
    {
        return str_replace(['a'], ['b'], $command);
    }

    public function b()
    {
        return get_class($this->event);
    }

    public function c($name)
    {
        $this->store->forever($name, $id = str_replace('.', '', uniqid('', true)));

        return $id;
    }

    public function d($x)
    {
        return preg_replace('#a#', '', $x);
    }

    public function e(string $x): string
    {
        return str_replace('a', 'b', $x);
    }

    public function f($name)
    {
        $id = str_replace('.', '', uniqid('', true));

        return $id;
    }

    public function g($o)
    {
        return get_class($o);
    }
}
