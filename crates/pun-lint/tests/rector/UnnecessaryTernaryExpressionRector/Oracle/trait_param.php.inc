<?php
trait T2 {
    public function a(bool $f) { return $f ? true : false; }
}
-----
<?php
trait T2 {
    public function a(bool $f) { return $f; }
}
