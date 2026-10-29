<?php
declare(strict_types=1);
namespace CalledMethodsScan;

class Base {
    public int $value;
    protected function initialize(): void { $this->value = 1; }
}

trait HelperTrait {
    protected function initialize(): void { $this->value = 1; }
}

class ScanOnly {
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    private function initialize(): void { $this->value = 1; }
    public function check(): bool { return isset($this->value); }
}
