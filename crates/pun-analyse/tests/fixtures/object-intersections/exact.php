<?php

class Service {}
class Child extends Service {}

function exactAlternatives(Service $broad, bool $flag): void {
    $service = new Service();
    $child = new Child();
    \PHPStan\dumpType($flag ? $service : $child);
    \PHPStan\dumpType($flag ? $child : $service);
    \PHPStan\dumpType($flag ? new Service() : new Service());
    \PHPStan\dumpType($flag ? $broad : new Child());
    \PHPStan\dumpType($flag ? new Child() : $broad);
    \PHPStan\dumpType($flag ? $broad : new Service());
    \PHPStan\dumpType($flag ? new Service() : $broad);
    $alias = $flag ? $service : $child;
    $copy = $alias;
    \PHPStan\dumpType($copy);
    $service = $child;
    \PHPStan\dumpType($service);
    if ($flag) {
        $joined = new Service();
    } else {
        $joined = new Child();
    }
    \PHPStan\dumpType($joined);
    if ($flag) {
        $reverse = new Child();
    } else {
        $reverse = new Service();
    }
    \PHPStan\dumpType($reverse);
    if ($flag) {
        $broad = new Child();
    }
    \PHPStan\dumpType($broad);
}

class Receiver extends Service {
    final public function __construct() {}
    public function combine(bool $flag): void {
        \PHPStan\dumpType($flag ? new self() : new static());
        \PHPStan\dumpType($flag ? new static() : new self());
        \PHPStan\dumpType($flag ? new self() : $this);
        \PHPStan\dumpType($flag ? $this : new self());
        \PHPStan\dumpType($flag ? new static() : $this);
        \PHPStan\dumpType($flag ? $this : new static());
    }
}

/** @template T */
class Box {
    /** @param T $value */
    public function __construct($value) {}
}

function exactGenericAlternatives(bool $flag): void {
    \PHPStan\dumpType($flag ? new Box(new Service()) : new Box(new Child()));
    \PHPStan\dumpType($flag ? new Box(new Child()) : new Box(new Service()));
}

/** @template-covariant T */
class CovariantBox {}

/** @param CovariantBox<int>|CovariantBox<1> $value */
function broadGenericAlternatives($value): void {
    \PHPStan\dumpType($value);
}

trait Marker {}
