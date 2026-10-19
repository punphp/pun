<?php

class C {
    public function tick(): void {
        static $never_touched = 0;
    }
}
