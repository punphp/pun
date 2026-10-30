<?php
declare(strict_types=1);
namespace CalledMethodsEnds;

class RecursiveConstructor {
    public int $value;
    public function __construct(bool $stop = false) {
        if ($stop) { return; }
        $this->__construct(true);
        isset($this->value);
        $this->initialize();
        isset($this->value);
    }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class DeadReturn {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        if (false) { return; }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}

class FalseFinalIf {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void { if (false) { $this->value = 1; } }
    public function check(): bool { return isset($this->value); }
}

class TrueFinalIf {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        if (true) { $this->value = 1; }
        elseif (true) { return; }
        else { return; }
    }
    public function check(): bool { return isset($this->value); }
}

class NestedFinalIf {
    public int $value;
    public function __construct() { $this->initialize(false, false); isset($this->value); }
    private function initialize(bool $first, bool $second): int {
        if ($first) {
            if ($second) { return $this->value = 1; }
            else { throw new \RuntimeException(); }
        } else { $this->value = 2; return 2; }
    }
    public function check(): bool { return isset($this->value); }
}

class ThrowFinally {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        try { throw new \RuntimeException(); }
        finally { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}

class ReturnNeverOperand {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): int { return throw new \RuntimeException(); }
    public function check(): bool { return isset($this->value); }
}

class AliasOverwrittenByArgument {
    public int $value;
    public function __construct(self $other) {
        $alias = $this;
        $alias->initialize($alias = $other);
        isset($this->value);
    }
    private function initialize(self $ignored): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class NullsafeAlias {
    public int $value;
    public function __construct(bool $stop) {
        $alias = $stop ? null : $this;
        $alias?->initialize();
        isset($this->value);
    }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class StaticHelper {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private static function initialize(): void {}
    public function check(): bool { return isset($this->value); }
}

class FinalEmptyBranch {
    public int $value;
    public function __construct() { $this->initialize(false); isset($this->value); }
    private function initialize(bool $stop): void {
        if ($stop) {} else { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}

class LocalHelper {
    public int $value;
    private function initialize(): void { $this->value = 1; }
    public function __construct() { $this->INITIALIZE(); isset($this->value); }
    public function check(): bool { return isset($this->value); }
}

class FreshNullsafeScope {
    public int $value;
    public function __construct() {
        $alias = $this;
        $alias?->initialize();
        isset($this->value);
    }
    private function initialize(?self $alias = null): void { $alias?->accept($this->value = 1); }
    private function accept(int $value): void {}
    public function check(): bool { return isset($this->value); }
}

class ThisReturningReceiver {
    public int $value;
    public function __construct() { $this->same()->initialize(); isset($this->value); }
    /** @return $this */
    private function same(): self { return $this; }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class SelfReturningReceiver {
    public int $value;
    public function __construct() { $this->same()->initialize(); isset($this->value); }
    private function same(): self { return $this; }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class StaticReturningReceiver {
    public int $value;
    public function __construct() { $this->same()->initialize(); isset($this->value); }
    private function same(): static { return $this; }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

interface ReceiverTag {}
class IntersectionReceiver {
    public int $value;
    public function __construct() {
        $alias = $this;
        if ($alias instanceof ReceiverTag) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class IntersectionStaticReceiver {
    public int $value;
    public function __construct() {
        $alias = $this->same();
        if ($alias instanceof ReceiverTag) {
            $alias->initialize();
            isset($this->value);
        }
        isset($this->value);
    }
    private function same(): static { return $this; }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class UnreachableLastAfterReturn {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): int {
        return $this->value = 1;
        $dead = 0;
    }
    public function check(): bool { return isset($this->value); }
}

class UnreachableLastAfterThrow {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        $this->value = 1;
        throw new \RuntimeException();
        $dead = 0;
    }
    public function check(): bool { return isset($this->value); }
}

class UnreachableBranchLast {
    public int $value;
    public function __construct() { $this->initialize(false); isset($this->value); }
    private function initialize(bool $stop): int {
        if ($stop) {
            return $this->value = 1;
            $dead = 0;
        } else { $this->value = 2; return 2; }
    }
    public function check(): bool { return isset($this->value); }
}
