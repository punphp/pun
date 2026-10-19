<?php
final class BoolVariadic {
    public function a(bool ...$flags) { return $flags ? true : false; }
}
-----
<?php
final class BoolVariadic {
    public function a(bool ...$flags) { return (bool) $flags; }
}
