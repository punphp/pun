<?php
final class Suppressed {
    public function a($from, $to) { return @rename($from, $to) === true; }
    public function b($from) { return @unlink($from) !== false; }
}
-----
<?php
final class Suppressed {
    public function a($from, $to) { return @rename($from, $to); }
    public function b($from) { return @unlink($from); }
}
