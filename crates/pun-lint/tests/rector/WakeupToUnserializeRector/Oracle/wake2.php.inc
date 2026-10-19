<?php

class A {
    public function __wakeup(){
        $a = 1;
        $b = 2;
    }
}

class B {
    /**
     * Doc.
     */
    #[Attr]
    final public static function &__wakeup(int $x = 1,
        $y) {}
}

class C {
    public function __wakeup()
    {
        // lead
        $this->x = 1; // tail
    }
}

class D {
    protected function __wakeup(  ) : void
    {
        $this->x = 1;
    }
}

class E {
    public function __wakeup(
        $a,
    ) {
        $this->x = 1;
    }
}

class F {
    public function __wakeup()
    {
        $this->x = 1;
        // after
    }
}

class G {
    public function __wakeup()
    {
        // one
        // two
    }
}
-----
<?php

class A {
    public function __unserialize(array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class B {
    /**
     * Doc.
     */
    #[Attr]
    final public static function &__unserialize(int $x = 1, $y, array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class C {
    public function __unserialize(array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class D {
    protected function __unserialize(array $data  ) : void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class E {
    public function __unserialize(
        $a, array $data,
    ): void {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class F {
    public function __unserialize(array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class G {
    public function __unserialize(array $data): void
    {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}
