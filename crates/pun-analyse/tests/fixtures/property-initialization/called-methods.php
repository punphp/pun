<?php
declare(strict_types=1);
namespace CalledMethods;

class Direct {
    public int $value;
    public function __construct() {
        isset($this->value);
        $this->initialize();
        isset($this->value);
        $this->initialize(); // cached summary
        isset($this->value);
    }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class NoHelper {
    public int $value;
    public function __construct() { isset($this->value); }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class AliasReceiver {
    public int $value;
    public function __construct() {
        $alias = $this;
        $alias->initialize();
        isset($this->value);
    }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class OtherReceiver {
    public int $value;
    public function __construct(self $other) {
        $other->initialize();
        isset($this->value);
    }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class DynamicName {
    public int $value;
    public function __construct() {
        $name = 'initialize';
        $this->$name();
        isset($this->value);
    }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class OutsideConstructor {
    public int $value;
    public function run(): bool { $this->initialize(); return isset($this->value); }
    private function initialize(): void { $this->value = 1; }
}

class ParentHelper {
    public int $value;
    protected function initialize(): void { $this->value = 1; }
}
class InheritedHelper extends ParentHelper {
    public function __construct() { $this->initialize(); isset($this->value); }
    public function check(): bool { return isset($this->value); }
}

class NestedHelper {
    public int $value;
    public function __construct() { $this->forward(); isset($this->value); }
    private function forward(): void { $this->initialize(); }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class RawEarlyReturn {
    public int $value;
    public function __construct() { $this->initialize(false); isset($this->value); }
    private function initialize(bool $stop): void {
        if ($stop) { return; }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}

class ThrowBranch {
    public int $value;
    public function __construct() { $this->initialize(false); isset($this->value); }
    private function initialize(bool $stop): void {
        if ($stop) { throw new \RuntimeException(); }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}

class ThrowEnd {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void { $this->value = 1; throw new \RuntimeException(); }
    public function check(): bool { return isset($this->value); }
}

class DeclaredNever {
    public int $value;
    public function __construct() { $this->initialize(); }
    private function initialize(): never { $this->value = 1; throw new \RuntimeException(); }
    public function check(): bool { return isset($this->value); }
}

class ReturnAssignment {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): int { return $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class EarlierReturnAssignment {
    public int $value;
    public function __construct() { $this->initialize(false); isset($this->value); }
    private function initialize(bool $stop): int {
        if ($stop) { return $this->value = 1; }
        return 0;
    }
    public function check(): bool { return isset($this->value); }
}

class FinalIfReturns {
    public int $value;
    public function __construct() { $this->initialize(false); isset($this->value); }
    private function initialize(bool $stop): int {
        if ($stop) { return $this->value = 1; }
        else { return 0; }
    }
    public function check(): bool { return isset($this->value); }
}

class FinalIfThrows {
    public int $value;
    public function __construct() { $this->initialize(false); isset($this->value); }
    private function initialize(bool $stop): void {
        if ($stop) { $this->value = 1; }
        else { throw new \RuntimeException(); }
    }
    public function check(): bool { return isset($this->value); }
}

class FinalElseIf {
    public int $value;
    public function __construct() { $this->initialize(0); isset($this->value); }
    private function initialize(int $mode): void {
        if ($mode === 0) { $this->value = 1; }
        elseif ($mode === 1) { $this->value = 2; }
        else { throw new \RuntimeException(); }
    }
    public function check(): bool { return isset($this->value); }
}

class FinallyRawReturn {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        try { return; }
        finally { $this->value = 1; }
    }
    public function check(): bool { return isset($this->value); }
}

class OnlyMarkers {
    public ?int $value;
    public int $other;
    public function __construct() {
        $local = 3;
        $this->initialize();
        isset($this->value);
        isset($this->other);
        // A helper local/value write is not a constructor local/value refinement.
        $copy = $local;
    }
    private function initialize(): void { $local = 'helper'; $this->value = null; }
    public function check(): bool { return isset($this->value); }
}

class ExistingYes {
    public int $value;
    public function __construct() {
        $this->value = 1;
        $this->initialize(false);
        isset($this->value);
    }
    private function initialize(bool $stop): void { if ($stop) { $this->value = 2; } }
    public function check(): bool { return isset($this->value); }
}

class ExistingMaybe {
    public int $value;
    public function __construct(bool $stop) {
        if ($stop) { $this->value = 1; }
        $this->initialize();
        isset($this->value);
    }
    private function initialize(): void { $this->value = 2; }
    public function check(): bool { return isset($this->value); }
}

class NestedFunction {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        $closure = function () { return; };
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}

class FinalBlock {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void { { $this->value = 1; } }
    public function check(): bool { return isset($this->value); }
}

class FiftyLines {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        $this->value = 1;
        // 02
        // 03
        // 04
        // 05
        // 06
        // 07
        // 08
        // 09
        // 10
        // 11
        // 12
        // 13
        // 14
        // 15
        // 16
        // 17
        // 18
        // 19
        // 20
        // 21
        // 22
        // 23
        // 24
        // 25
        // 26
        // 27
        // 28
        // 29
        // 30
        // 31
        // 32
        // 33
        // 34
        // 35
        // 36
        // 37
        // 38
        // 39
        // 40
        // 41
        // 42
        // 43
        // 44
        // 45
        // 46
        // 47
        // 48
        // 49
    }
    public function check(): bool { return isset($this->value); }
}

class FiftyOneLines {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {
        $this->value = 1;
        // 02
        // 03
        // 04
        // 05
        // 06
        // 07
        // 08
        // 09
        // 10
        // 11
        // 12
        // 13
        // 14
        // 15
        // 16
        // 17
        // 18
        // 19
        // 20
        // 21
        // 22
        // 23
        // 24
        // 25
        // 26
        // 27
        // 28
        // 29
        // 30
        // 31
        // 32
        // 33
        // 34
        // 35
        // 36
        // 37
        // 38
        // 39
        // 40
        // 41
        // 42
        // 43
        // 44
        // 45
        // 46
        // 47
        // 48
        // 49
        // 50
    }
    public function check(): bool { return isset($this->value); }
}

new class {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
};

function declareNestedClass(): void {
    class NestedClass {
        public int $value;
        public function __construct() { $this->initialize(); isset($this->value); }
        private function initialize(): void { $this->value = 1; }
        public function check(): bool { return isset($this->value); }
    }
}

abstract class AbstractHelper {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    abstract protected function initialize(): void;
    public function check(): bool { return isset($this->value); }
}

class EmptyHelper {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void {}
    public function check(): bool { return isset($this->value); }
}
