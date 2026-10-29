<?php
declare(strict_types=1);

class MarkerFlows {
    public int $value;
    public int $other;
    public function branch(bool $b): bool {
        if ($b) { $this->value = 1; }
        return isset($this->value);
    }
    public function whileLoop(bool $b): bool {
        while ($b) { $this->value = 1; $b = false; }
        return isset($this->value);
    }
    public function doLoop(bool $b): bool {
        do { $this->value = 1; } while ($b);
        return isset($this->value);
    }
    public function forLoop(bool $b): bool {
        for (; $b; $b = false) { $this->value = 1; }
        return isset($this->value);
    }
    public function foreachLoop(array $items): bool {
        foreach ($items as $item) { $this->value = 1; }
        return isset($this->value);
    }
    public function caught(bool $b): bool {
        try { if ($b) { throw new RuntimeException(); } $this->value = 1; }
        catch (RuntimeException $e) {}
        return isset($this->value);
    }
    public function finallyAssigned(): bool {
        try { $this->other = 1; } finally { $this->value = 1; }
        return isset($this->value);
    }
    public function alias(): bool {
        $alias = $this;
        $alias->value = 1;
        unset($this->value);
        return isset($this->value);
    }
    public function foreign(MarkerFlows $object): bool {
        $object->value = 1;
        return isset($this->value);
    }
    public function nested(): bool {
        $closure = function (): bool { $this->value = 1; return isset($this->value); };
        return isset($this->value);
    }
    public function nestedInherited(): bool {
        $this->value = 1;
        $closure = function (): bool { return isset($this->value); };
        $arrow = fn(): bool => isset($this->value);
        return isset($this->value);
    }
    public function twoProperties(bool $b): bool {
        $this->value = 1;
        if ($b) { $this->other = 1; }
        return isset($this->value, $this->other);
    }
    public function switchCases(int $i): bool {
        switch ($i) {
            case 0: $this->value = 1; break;
            default: $this->value = 2; break;
        }
        return isset($this->value);
    }
}

class NestedConstructor {
    public int $value;
    public function __construct(bool $b) {
        $closure = function (): void { return; };
        function isolatedFunction(): void { return; }
        if ($b) { $this->value = 1; return; }
        $this->value = 2;
    }
    public function check(): bool { return isset($this->value); }
}

class RawFinallyConstructor {
    public int $value;
    public function __construct(bool $b) {
        try { if ($b) { return; } } finally { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}

class OnlyThrowConstructor {
    public int $value;
    public function __construct() { $this->value = 1; throw new RuntimeException(); }
    public function check(): bool { return isset($this->value); }
}

class InheritedSleep {
    public function __sleep(): array { return []; }
}
class ChildSleep extends InheritedSleep {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
trait MarkerSerialize {
    public function __serialize(): array { return []; }
}
class TraitSerialization {
    use MarkerSerialize;
    public int $value;
    public function __construct() { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class OldSerialization implements Serializable {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function serialize(): string { return ''; }
    public function unserialize(string $data): void {}
    public function check(): bool { return isset($this->value); }
}
class OrderedMethods {
    public int $value;
    public function check(): bool { return isset($this->value); }
    public static function checkStatic(OrderedMethods $object): bool { return isset($object->value); }
    public function __construct() { $this->value = 1; }
    public function notAnInitializer(): void { $this->other = 1; }
    public int $other;
    public function otherCheck(): bool { return isset($this->other); }
}

class BareUnserialize {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function unserialize(string $data): void {}
    public function check(): bool { return isset($this->value); }
}
class MagicUnserialize {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function __unserialize(array $data): void {}
    public function check(): bool { return isset($this->value); }
}
