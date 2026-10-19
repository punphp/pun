<?php

class A
{
    public function __sleep()
    {
        if ($x) {
            return ['a'];
        }
        return;
    }
}

class B
{
    public function __sleep()
    {
        return $this->fields;
    }
}

abstract class C
{
    abstract public function __sleep();
}

class D
{
    public function __sleep()
    {
        return [...$this->a, 'b'];
    }
}

class E
{
    public function __sleep()
    {
        try {
            return ['a'];
        } catch (Exception $e) {
            throw $e;
        }
    }
}

class F
{
    public function __sleep()
    {
        try {
            $x = 1;
        } catch (Exception $e) {
            exit(1);
        } finally {
            return ['a'];
        }
    }
}

class G
{
    public function __sleep()
    {
        try {
            return ['a'];
        } catch (Exception $e) {
            $x = 1;
        }
    }
}

class H
{
    public function __sleep()
    {
        if ($x) {
            return ['a'];
        } elseif ($y) {
            die();
        } else {
            return ['b'];
        }
    }
}

class I
{
    public function __sleep()
    {
        if ($x) {
            return ['a'];
        } elseif ($y) {
            $z = 1;
        } else {
            return ['b'];
        }
    }
}

class J
{
    public function __sleep()
    {
        try {
            $x = 1;
        } finally {
            $y = 1;
        }
        yield 1;
        return ['a'];
    }
}

class K
{
    public function __sleep()
    {
        return [1];
        return ['a', 'b'];
    }
}

class L
{
    public function __sleep()
    {
        foo();
        return ['a'];
    }
}
-----
<?php

class A
{
    public function __sleep()
    {
        if ($x) {
            return ['a'];
        }
        return;
    }
}

class B
{
    public function __sleep()
    {
        return $this->fields;
    }
}

abstract class C
{
    abstract public function __sleep();
}

class D
{
    public function __serialize(): array
    {
        return ['b' => $this->b];
    }
}

class E
{
    public function __serialize(): array
    {
        try {
            return ['a' => $this->a];
        } catch (Exception $e) {
            throw $e;
        }
    }
}

class F
{
    public function __serialize(): array
    {
        try {
            $x = 1;
        } catch (Exception $e) {
            exit(1);
        } finally {
            return ['a' => $this->a];
        }
    }
}

class G
{
    public function __sleep()
    {
        try {
            return ['a'];
        } catch (Exception $e) {
            $x = 1;
        }
    }
}

class H
{
    public function __serialize(): array
    {
        if ($x) {
            return ['a' => $this->a];
        } elseif ($y) {
            die();
        } else {
            return ['b' => $this->b];
        }
    }
}

class I
{
    public function __sleep()
    {
        if ($x) {
            return ['a'];
        } elseif ($y) {
            $z = 1;
        } else {
            return ['b'];
        }
    }
}

class J
{
    public function __sleep()
    {
        try {
            $x = 1;
        } finally {
            $y = 1;
        }
        yield 1;
        return ['a'];
    }
}

class K
{
    public function __serialize(): array
    {
        return [1];
        return ['a' => $this->a, 'b' => $this->b];
    }
}

class L
{
    public function __serialize(): array
    {
        foo();
        return ['a' => $this->a];
    }
}
