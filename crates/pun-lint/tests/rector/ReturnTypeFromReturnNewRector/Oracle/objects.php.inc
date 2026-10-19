<?php

namespace App;

interface Marker
{
}

class Base
{
}

class Factory
{
    public function lazy(\ReflectionClass $class)
    {
        $instance = $class->newLazyGhost(function ($o) {
        });

        return $instance;
    }

    public function make($x)
    {
        if ($x) {
            return new class extends Base implements Marker {
            };
        }

        return new Base();
    }

    public function marker($x)
    {
        if ($x) {
            return new class implements Marker {
            };
        }

        return new class extends Base {
        };
    }

    public function overwritten($x)
    {
        $a = new Base();
        $a = $x;

        return $a;
    }

    public function created()
    {
        $a = new Base();
        $a->foo = 1;

        return $a;
    }

    public function selfNew()
    {
        $a = new self();

        return $a;
    }
}
-----
<?php

namespace App;

interface Marker
{
}

class Base
{
}

class Factory
{
    public function lazy(\ReflectionClass $class): object
    {
        $instance = $class->newLazyGhost(function ($o) {
        });

        return $instance;
    }

    public function make($x): \App\Base
    {
        if ($x) {
            return new class extends Base implements Marker {
            };
        }

        return new Base();
    }

    public function marker($x): \App\Marker|\App\Base
    {
        if ($x) {
            return new class implements Marker {
            };
        }

        return new class extends Base {
        };
    }

    public function overwritten($x)
    {
        $a = new Base();
        $a = $x;

        return $a;
    }

    public function created(): \App\Base
    {
        $a = new Base();
        $a->foo = 1;

        return $a;
    }

    public function selfNew()
    {
        $a = new self();

        return $a;
    }
}
