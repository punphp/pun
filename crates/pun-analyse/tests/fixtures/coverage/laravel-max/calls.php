<?php

namespace LaravelMax\Calls;

class Calls
{
    /** @return mixed */
    public function attribute()
    {
        return 1;
    }

    public function route(): ?string
    {
        return null;
    }

    public function remembered(): void
    {
        $connection = $this->route() ?? null;
        $this->route()->foo();
        $queue = $this->attribute() ?? null;
        $key = 'queue:' . $this->attribute();
    }

    public function className(object $object, \stdClass $class): void
    {
        if ($object instanceof \Countable) {
            $this->takesInt(get_class($object));
        }
        $this->takesInt(get_class($class));
    }

    public function takesInt(int $int): void
    {
    }
}
