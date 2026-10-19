<?php

// Rector also rewrites `$this->getName()` inside the anonymous class, whose `$this` is another object; Pun leaves
// calls in nested classes alone.
final class B
{
    private $name;

    public function run()
    {
        $u = new class {
            public function run()
            {
                return $this->getName();
            }
        };
        return $this->getName();
    }

    public function getName()
    {
        return $this->name;
    }
}
-----
<?php

// Rector also rewrites `$this->getName()` inside the anonymous class, whose `$this` is another object; Pun leaves
// calls in nested classes alone.
final class B
{
    private $name;

    public function run()
    {
        $u = new class {
            public function run()
            {
                return $this->getName();
            }
        };
        return $this->name;
    }

    public function getName()
    {
        return $this->name;
    }
}
