<?php
trait T {
    public function make(): object {
        return new class {
            private function unused(): void {}
            private int $x = 1;
        };
    }
}
class A { use T; }
class B { use T; }
enum E { use T; }
