<?php
declare(strict_types=1);
namespace CalledMethodsScan;

class InheritedFromScan extends Base {
    public function __construct() { $this->initialize(); isset($this->value); }
    public function check(): bool { return isset($this->value); }
}

class TraitFromScan {
    use HelperTrait;
    public int $value;
    public function __construct() { $this->initialize(); isset($this->value); }
    public function check(): bool { return isset($this->value); }
}

function createScanned(): ScanOnly { return new ScanOnly(); }
