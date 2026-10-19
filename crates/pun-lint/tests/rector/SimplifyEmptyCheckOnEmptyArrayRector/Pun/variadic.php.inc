<?php
final class Variadics {
    public function a(string ...$guards) { return empty($guards); }
    public function b(...$items) { return ! empty($items); }
}
-----
<?php
final class Variadics {
    public function a(string ...$guards) { return $guards === []; }
    public function b(...$items) { return $items !== []; }
}
