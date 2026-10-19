<?php

class A
{
    private $x;

    // lead
    public function __WAKEUP(): void
    {
        // first
        $this->x = 1;

        $this->y = 2; // tail
    }
}

class B
{
    public function __wakeup($a)
    {
        $this->x = 1;
    }

    public function __unserialize(array $data): void
    {
    }
}

trait T
{
    public function __wakeup()
    {
        $this->x = 1;
    }
}

final class C
{
    public function __wakeup()
    {
        // only a comment
    }
}

$a = new class {
    public function __wakeup()
    {
        $this->x = 1;
    }
};
-----
<?php

class A
{
    private $x;

    // lead
    public function __unserialize(array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class B
{
    public function __wakeup($a)
    {
        $this->x = 1;
    }

    public function __unserialize(array $data): void
    {
    }
}

trait T
{
    public function __wakeup()
    {
        $this->x = 1;
    }
}

final class C
{
    public function __unserialize(array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

$a = new class {
    public function __unserialize(array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
};
