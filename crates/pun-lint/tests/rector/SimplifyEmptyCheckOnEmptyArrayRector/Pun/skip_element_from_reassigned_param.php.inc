<?php
final class Reassigned {
    /** @var array */
    private $u = [];
    public function push(array $v) { $v = $this->merge($v); $this->u[] = $v; }
    private function merge($v) { return $v; }
    public function a() { return empty($this->u); }
}
