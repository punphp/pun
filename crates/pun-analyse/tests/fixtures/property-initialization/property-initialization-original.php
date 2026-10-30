<?php
declare(strict_types=1);

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
        if ($skip) { throw new RuntimeException(); }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}

class AssignedReturn {
    public int $value;
    public function __construct(bool $skip) {
        if ($skip) { $this->value = 2; return; }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}

class AllReturn {
    public int $value;
    public function __construct(bool $skip) {
        if ($skip) { $this->value = 2; return; }
        $this->value = 1;
        return;
    }
    public function check(): bool { return isset($this->value); }
}

class ConditionalOnly {
    public int $value;
    public function __construct(bool $skip) {
        if ($skip) { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}

class InsideBranches {
    public int $value;
    public function check(bool $skip): bool {
        if ($skip) {
            $this->value = 1;
            return isset($this->value);
        }
        return isset($this->value);
    }
}

class Promoted {
    public function __construct(public int $value) {
        if (isset($this->value)) {}
    }
    public function check(): bool { return isset($this->value); }
}

class Serialized {
    public int $value;
    public function __construct() { $this->value = 1; }
    public function __serialize(): array { return []; }
    public function check(): bool { return isset($this->value); }
    public function local(): bool { $this->value = 1; return isset($this->value); }
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
    public function ownCheck(): bool { return isset($this->value); }
}

class ParentCall extends ParentInitialized {
    public function __construct() { parent::__construct(); }
    public function ownCheck(): bool { return isset($this->value); }
}

class InheritedLocal extends ParentInitialized {
    public function __construct() { $this->value = 1; }
    public function ownCheck(): bool { return isset($this->value); }
    public function local(): bool { $this->value = 1; return isset($this->value); }
}

class DirectHelper {
    public int $value;
    public function __construct() { $this->initialize(); }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class IndirectHelper {
    public int $value;
    public function __construct() { $this->forward(); }
    private function forward(): void { $this->initialize(); }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class ReturnFinally {
    public int $value;
    public function __construct() {
        try { return; }
        finally { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}

class Reordered {
    public int $value;
    public function check(): bool { return isset($this->value); }
    public function __construct() { $this->value = 1; }
}

class UnsetAfterAssignment {
    public int $value;
    public function __construct() { $this->value = 1; unset($this->value); }
    public function check(): bool { return isset($this->value); }
    public function local(): bool { $this->value = 2; unset($this->value); return isset($this->value); }
}

class AliasThis {
    public int $value;
    public function __construct() { $self = $this; $self->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class CoalesceAssignment {
    public int $value;
    public function __construct() { $this->value ??= 1; }
    public function check(): bool { return isset($this->value); }
}

class CaughtReturn {
    public int $value;
    public function __construct(bool $skip) {
        try {
            if ($skip) { throw new RuntimeException(); }
            $this->value = 1;
        } catch (RuntimeException $e) { return; }
    }
    public function check(): bool { return isset($this->value); }
}

class BothBranches {
    public int $value;
    public function __construct(bool $skip) {
        if ($skip) { $this->value = 1; } else { $this->value = 2; }
    }
    public function check(): bool { return isset($this->value); }
}

class PromotedReadonlySerialized {
    public function __construct(public readonly int $value) {}
    public function __serialize(): array { return []; }
    public function check(): bool { return isset($this->value); }
}

class PromotedMutableSerialized {
    public function __construct(public int $value) {}
    public function __serialize(): array { return []; }
    public function check(): bool { return isset($this->value); }
}
