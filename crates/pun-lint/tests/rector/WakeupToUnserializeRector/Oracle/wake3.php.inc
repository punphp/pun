<?php

class A {
    public function __wakeup()
    {
        // lead
        $this->x = 1;
    }
}

class B {
    #[Attr]
    public function __wakeup( ) {
        $this->x = 1;
    }
}

class C {
    public function __wakeup($a, $b) { $this->x = 1; }
}

class D {
    public function __wakeup(
        $a,
        $b
    ) {
        $this->x = 1;
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
    #[Attr]
    public function __unserialize(array $data ): void {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}

class C {
    public function __unserialize($a, $b, array $data): void { foreach ($data as $property => $value) {
        if (property_exists($this, $property)) {
            $this->{$property} = $value;
        }
    } }
}

class D {
    public function __unserialize(
        $a,
        $b,
        array $data
    ): void {
        foreach ($data as $property => $value) {
            if (property_exists($this, $property)) {
                $this->{$property} = $value;
            }
        }
    }
}
