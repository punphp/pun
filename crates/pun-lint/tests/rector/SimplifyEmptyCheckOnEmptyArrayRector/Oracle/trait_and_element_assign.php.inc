<?php
trait T1 {
    public function a(array $x) { return empty($x); }
    public function b(bool $f) { return $f === true; }
}
final class C1 {
    /** @var array */
    private $s = [];
    /** @var array */
    private $t = [];
    public function push($v) { $this->s[] = $v; }
    public function pushInt(int $v) { $this->t[] = $v; }
    public function a() { return empty($this->s); }
    public function b() { return empty($this->t); }
}
-----
<?php
trait T1 {
    public function a(array $x) { return $x === []; }
    public function b(bool $f) { return $f === true; }
}
final class C1 {
    /** @var array */
    private $s = [];
    /** @var array */
    private $t = [];
    public function push($v) { $this->s[] = $v; }
    public function pushInt(int $v) { $this->t[] = $v; }
    public function a() { return empty($this->s); }
    public function b() { return $this->t === []; }
}
