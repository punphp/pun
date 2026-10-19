<?php

namespace App;

use Foo\Bar;

class A
{
    public function run(Bar $bar, $plain)
    {
        call_user_func([$this, 'go']);
        \call_user_func([$this, 'go'], 1, $x);
        call_user_func(array($this, 'go'), [
            'a' => 1,
        ]);
        $f = function () {
            return call_user_func([$this, 'go']);
        };
        $g = static fn () => call_user_func([$this, 'go']);
        call_user_func([$bar, 'go']);
        call_user_func([$plain, 'go']);
        $made = new Bar();
        call_user_func([$made, 'go'], $made);
        call_user_func([new Bar(), 'go']);
        call_user_func([$this, $name]);
        call_user_func([$this, 'go', 3]);
        call_user_func([$this]);
        call_user_func('strlen', 'x');
        call_user_func(...[[$this, 'go']]);
        call_user_func(...);
        call_user_func();
        call_user_func_array([$this, 'go'], []);
        call_user_func(['k' => $this, 'go']);
        return call_user_func([$this->prop, 'go']);
    }

    public static function stat()
    {
        call_user_func([$this, 'go']);
    }
}

function f()
{
    call_user_func([$this, 'go']);
}

call_user_func([$this, 'go']);
-----
<?php

namespace App;

use Foo\Bar;

class A
{
    public function run(Bar $bar, $plain)
    {
        $this->go();
        $this->go(1, $x);
        $this->go([
            'a' => 1,
        ]);
        $f = function () {
            return $this->go();
        };
        $g = static fn () => call_user_func([$this, 'go']);
        $bar->go();
        call_user_func([$plain, 'go']);
        $made = new Bar();
        $made->go($made);
        call_user_func([new Bar(), 'go']);
        call_user_func([$this, $name]);
        call_user_func([$this, 'go', 3]);
        call_user_func([$this]);
        call_user_func('strlen', 'x');
        call_user_func(...[[$this, 'go']]);
        call_user_func(...);
        call_user_func();
        call_user_func_array([$this, 'go'], []);
        $this->go();
        return call_user_func([$this->prop, 'go']);
    }

    public static function stat()
    {
        call_user_func([$this, 'go']);
    }
}

function f()
{
    call_user_func([$this, 'go']);
}

call_user_func([$this, 'go']);
