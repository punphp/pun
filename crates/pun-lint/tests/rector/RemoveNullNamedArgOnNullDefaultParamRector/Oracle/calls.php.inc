<?php

namespace App;

function withNulls(?int $a = null, ?int $b = null, int $c = 1, ?string $d = null)
{
}

function readsArgs($a = null)
{
    return func_get_args();
}

final class Box
{
    public function put(?int $a = null, ?int $b = null)
    {
    }

    public function run(array $args)
    {
        $this->put(a: null, b: null);
        $this->put(b: null, a: 1);
        $this->put(1, b: null);
        $this->put(null, null);
        $this->put(...$args);
    }
}

withNulls(a: null, c: 2, d: null);
withNulls(c: null, b: null);
withNulls(1, d: NULL);
withNulls(x: null);
readsArgs(a: null);
withNulls(1, b: 2);
-----
<?php

namespace App;

function withNulls(?int $a = null, ?int $b = null, int $c = 1, ?string $d = null)
{
}

function readsArgs($a = null)
{
    return func_get_args();
}

final class Box
{
    public function put(?int $a = null, ?int $b = null)
    {
    }

    public function run(array $args)
    {
        $this->put();
        $this->put(a: 1);
        $this->put(1);
        $this->put(null, null);
        $this->put(...$args);
    }
}

withNulls(c: 2);
withNulls(c: null);
withNulls(1);
withNulls(x: null);
readsArgs(a: null);
withNulls(1, b: 2);
