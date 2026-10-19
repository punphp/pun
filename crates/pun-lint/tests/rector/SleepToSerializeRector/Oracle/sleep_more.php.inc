<?php

class A
{
    public function __SLEEP()
    {
        if ($this->a) {
            return ['a', 1, "b"];
        }

        return [
            'c',
            'd',
        ];
    }
}

class B
{
    public function __sleep(): ?array
    {
        return array('x', 'y');
    }
}

class C
{
    public function __sleep(): ARRAY
    {
        return ['x'];
    }
}

class D
{
    public function __sleep()
    {
        if ($this->a) {
            return ['a'];
        }
    }
}

class E
{
    public function __sleep()
    {
        return [1, 2];
    }
}

class F
{
    public function __sleep()
    {
        return ['a-b', 'k' => 'v', ...$x];
    }

    public function __serialize(): array
    {
        return [];
    }
}

class G
{
    public function __sleep()
    {
        $f = function () {
            return 1;
        };
        return ['a'];
    }
}

class I
{
    public function __sleep()
    {
        try {
            return ['a'];
        } finally {
        }
    }
}

class J
{
    public function __sleep()
    {
        if ($x) {
            return ['a'];
        } else {
            return ['b'];
        }
    }
}
-----
<?php

class A
{
    public function __serialize(): array
    {
        if ($this->a) {
            return ['a' => $this->a, "b" => $this->b];
        }

        return [
            'c' => $this->c,
            'd' => $this->d,
        ];
    }
}

class B
{
    public function __serialize(): array
    {
        return array('x' => $this->x, 'y' => $this->y);
    }
}

class C
{
    public function __sleep(): ARRAY
    {
        return ['x'];
    }
}

class D
{
    public function __sleep()
    {
        if ($this->a) {
            return ['a'];
        }
    }
}

class E
{
    public function __sleep()
    {
        return [1, 2];
    }
}

class F
{
    public function __sleep()
    {
        return ['a-b', 'k' => 'v', ...$x];
    }

    public function __serialize(): array
    {
        return [];
    }
}

class G
{
    public function __serialize(): array
    {
        $f = function () {
            return 1;
        };
        return ['a' => $this->a];
    }
}

class I
{
    public function __serialize(): array
    {
        try {
            return ['a' => $this->a];
        } finally {
        }
    }
}

class J
{
    public function __serialize(): array
    {
        if ($x) {
            return ['a' => $this->a];
        } else {
            return ['b' => $this->b];
        }
    }
}
