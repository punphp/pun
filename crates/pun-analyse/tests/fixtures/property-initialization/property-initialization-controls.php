<?php
declare(strict_types=1);

function propertyInitializationValue(): int { return 1; }

class SingleVisitMarker {
    public int $value;
    public function __construct() { $this->value = propertyInitializationValue(); }
    public function check(): bool { return isset($this->value); }
}
class ImpossibleReturnMarker {
    public int $value;
    public function __construct() { if (false) { return; } $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class DoReturnMarker {
    public int $value;
    public function __construct(bool $b) {
        do { if ($b) { return; } $this->value = 1; } while (false);
    }
    public function check(): bool { return isset($this->value); }
}
class ThrowFinallyMarker {
    public int $value;
    public function __construct() {
        try { throw new RuntimeException(); } finally { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}
class TwoRawReturnsMarker {
    public int $value;
    public function __construct() {
        try { return; } finally { $this->value = 1; return; }
    }
    public function check(): bool { return isset($this->value); }
}
class NestedFinallyMarker {
    public int $value;
    public function check(): bool {
        do { try { try { break; } finally {} } finally { $this->value = 1; } } while (false);
        return isset($this->value);
    }
}
class ContinueSwitchMarker {
    public int $value;
    public function check(int $i): bool {
        switch ($i) { default: try { continue; } finally { $this->value = 1; } }
        return isset($this->value);
    }
}
class ThrowCaughtMarker {
    public int $value;
    public function __construct(bool $b) {
        try { if ($b) { throw new RuntimeException(); } $this->value = 1; }
        catch (RuntimeException $e) { $this->value = 2; }
    }
    public function check(): bool { return isset($this->value); }
}
class FinallyReturnsMarker {
    public int $value;
    public function __construct() { try {} finally { $this->value = 1; return; } }
    public function check(): bool { return isset($this->value); }
}
class NestedLoopArrowMarker {
    public int $value;
    public function check(bool $b): bool {
        while ($b) { $arrow = fn(): bool => isset($this->value); $this->value = 1; }
        return isset($this->value);
    }
}
class ReturnOperandMarker {
    public int $value;
    public function __construct() { return $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class ReturnThrowMarker {
    public int $value;
    public function __construct() { $this->value = 1; return throw new RuntimeException(); }
    public function check(): bool { return isset($this->value); }
}
trait DeclaredMarkerProperty { public int $value; }
class TraitPropertyMarker {
    use DeclaredMarkerProperty;
    public function __construct() { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
class RedeclaredMarkerProperty {
    public int $value;
}
class RedeclaredMarkerChild extends RedeclaredMarkerProperty {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

function stopPropertyInitialization(): never { throw new RuntimeException(); }
class LateTerminatingSwitchMarker {
    public int $value;
    public int $other;
    public function middleGroup(int $i): bool {
        switch ($i) {
            case 0: $this->value = 1; stopPropertyInitialization();
            case 1: $this->other = 1;
            default: break;
        }
        return isset($this->value, $this->other);
    }
    public function lastGroup(int $i): bool {
        switch ($i) {
            case 0: $this->value = 1; stopPropertyInitialization();
            default: $this->other = 1; break;
        }
        return isset($this->value, $this->other);
    }
}
