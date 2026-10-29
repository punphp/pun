<?php
declare(strict_types=1);

namespace PropertyInitializationConsumers;

class EarlyReturn {
    public int $value;
    public function __construct(bool $skip) {
        if ($skip) { return; }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}
class ThrowExit {
    public int $value;
    public function __construct(bool $skip) {
        if ($skip) { throw new \RuntimeException(); }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}
class AssignedReturns {
    public int $value;
    public function __construct(bool $skip) {
        if ($skip) { $this->value = 2; return; }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}
class RawReturnFinally {
    public int $value;
    public function __construct(bool $skip) {
        try { if ($skip) { return; } } finally { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}
class Reordered {
    public int $value;
    public function check(): bool { return isset($this->value); }
    public function __construct() { $this->value = 1; }
}
class Promoted {
    public function __construct(public int $value) {
        echo isset($this->value);
    }
    public function check(): bool { return isset($this->value); }
}
class AliasAssignment {
    public int $value;
    public function __construct() { $self = $this; $self->value = 1; }
    public function check(): bool { return isset($this->value); }
    public function local(): bool { $self = $this; $self->value = 1; return isset($this->value); }
}
class LocalScopes {
    public int $value;
    public function branch(bool $b): bool {
        if ($b) { $this->value = 1; return isset($this->value); }
        return isset($this->value);
    }
    public function partial(bool $b): bool {
        if ($b) { $this->value = 1; }
        return isset($this->value);
    }
    public function both(bool $b): bool {
        if ($b) { $this->value = 1; } else { $this->value = 2; }
        return isset($this->value);
    }
    public function loop(bool $b): bool {
        do { $this->value = 1; } while ($b);
        return isset($this->value);
    }
    public function zeroIterations(bool $b): bool {
        while ($b) { $this->value = 1; $b = false; }
        return isset($this->value);
    }
    public function caught(bool $b): bool {
        try { if ($b) { throw new \RuntimeException(); } $this->value = 1; }
        catch (\RuntimeException $e) {}
        return isset($this->value);
    }
    public function nested(): bool {
        $closure = function (): bool { $this->value = 1; return isset($this->value); };
        return isset($this->value);
    }
    public function aliasRead(): bool {
        $this->value = 1;
        $self = $this;
        return isset($self->value);
    }
    public function dynamicRead(): bool {
        $this->value = 1;
        $name = 'value';
        return isset($this->$name);
    }
}
class UnsetPreservesMarker {
    public int $value;
    public function __construct() { $this->value = 1; unset($this->value); }
    public function check(): bool { return isset($this->value); }
    public function local(): bool { $this->value = 1; unset($this->value); return isset($this->value); }
}
class CustomSerialization {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function __serialize(): array { return []; }
    public function check(): bool { return isset($this->value); }
    public function local(): bool { $this->value = 1; return isset($this->value); }
}
class SleepParent { public function __sleep(): array { return []; } }
class InheritedSerialization extends SleepParent {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class WakeupOnly {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function __wakeup(): void {}
    public function check(): bool { return isset($this->value); }
}
class ParentInitialized {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class InheritedOnly extends ParentInitialized {
    public function check(): bool { return isset($this->value); }
}
class InheritedWrite extends ParentInitialized {
    public function __construct() { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
    public function local(): bool { $this->value = 1; return isset($this->value); }
}
class Coalescing {
    public int $value;
    public ?int $nullable;
    public function __construct() { $self = $this; $self->value = 1; $self->nullable = null; }
    public function value(): int { return $this->value ?? 2; }
    public function assign(): int { return $this->value ??= 2; }
    public function nullable(): int { return $this->nullable ?? 2; }
    public function nullableIsset(): bool { return isset($this->nullable); }
}
class NullCoalescingReturn {
    public int $value;
    public function __construct(bool $skip) { if ($skip) { return; } $this->value = 1; }
    public function value(): int { return $this->value ?? 2; }
    public function assign(): int { return $this->value ??= 2; }
}
class OnlyThrow {
    public int $value;
    public function __construct() { $this->value = 1; throw new \RuntimeException(); }
    public function check(): bool { return isset($this->value); }
}
