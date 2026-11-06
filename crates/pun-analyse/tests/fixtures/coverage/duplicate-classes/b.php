<?php

class Dup {
    public function g($x) { return 2; }
    /** @phpstan-pure */
    public function h(): void { echo 1; }
}
