<?php
declare(strict_types=1);
namespace CalledMethodConsumers;

class DirectHelper {
    public int $value;
    public function check(): bool { return isset($this->value); }
    public function value(): int { return $this->value ?? 2; }
    public function assign(): int { return $this->value ??= 2; }
    public function __construct() { $this->initialize(); }
    private function initialize(): void { $this->value = 1; }
}

class NoHelper {
    public int $value;
    public function check(): bool { return isset($this->value); }
    public function value(): int { return $this->value ?? 2; }
}

class AliasHelper {
    public int $value;
    public function __construct() { $alias = $this; $alias->initialize(); }
    private function initialize(): void { $alias = $this; $alias->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class NestedHelper {
    public int $value;
    public function __construct() { $this->forward(); }
    private function forward(): void { $this->initialize(); }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}

class EarlyReturnHelper {
    public int $value;
    public function __construct() { $this->initialize(false); }
    private function initialize(bool $stop): void {
        if ($stop) { return; }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}

class ThrowExitHelper {
    public int $value;
    public function __construct() { $this->initialize(false); }
    private function initialize(bool $stop): void {
        if ($stop) { throw new \RuntimeException(); }
        $this->value = 1;
    }
    public function check(): bool { return isset($this->value); }
}
