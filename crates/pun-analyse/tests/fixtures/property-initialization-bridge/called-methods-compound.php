<?php
declare(strict_types=1);
namespace CalledMethodCompound;

class AOwner {
    public int $value;
    public function __construct(bool $flag, bool $reverse) {
        if ($reverse) {
            $alias = $flag ? new ZOther : $this;
            $alias->initialize();
            isset($this->value);
            return;
        }
        $alias = $flag ? $this : new ZOther;
        $alias->initialize();
        isset($this->value);
    }
    public function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class ZOther { public function initialize(): void {} }

class ZOwner {
    public int $value;
    public function __construct(bool $flag, bool $reverse) {
        if ($reverse) {
            $alias = $flag ? new AOther : $this;
            $alias->initialize();
            isset($this->value);
            return;
        }
        $alias = $flag ? $this : new AOther;
        $alias->initialize();
        isset($this->value);
    }
    public function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class AOther { public function initialize(): void {} }

interface ZNarrow { public function initialize(): void; }
class AIntersectionWide {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof ZNarrow) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    public function initialize(?int $optional = null): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

interface AWide { public function initialize(?int $optional = null): void; }
class ZIntersectionNarrow {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof AWide) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    public function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

interface ANarrow { public function initialize(): void; }
class ZIntersectionWide {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof ANarrow) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    public function initialize(?int $optional = null): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

interface ZWide { public function initialize(?int $optional = null): void; }
class AIntersectionNarrow {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof ZWide) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    public function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

interface EqualContract { public function initialize(): void; }
class EqualIntersection {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof EqualContract) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    public function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

interface VariadicContract { public function initialize(int ...$values): void; }
class FixedIntersection {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof VariadicContract) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    public function initialize(?int $first = null, ?int $second = null): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

interface FixedContract { public function initialize(?int $first = null, ?int $second = null): void; }
class VariadicIntersection {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof FixedContract) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    public function initialize(int ...$values): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class NoMethod {}
class FilteredUnion {
    public int $value;
    public function __construct(bool $flag) {
        $alias = $flag ? new NoMethod : $this;
        $alias->initialize();
        isset($this->value);
    }
    public function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class ScalarUnion {
    public int $value;
    public function __construct(bool $flag) {
        $alias = $flag ? 1 : $this;
        $alias->initialize();
        isset($this->value);
    }
    public function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
