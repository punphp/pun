<?php

class A {
    private bool $enableFoo = false;
    private bool $enableAll = false;

    public function configure(bool $enableFoo): void {
        $this->enableFoo = $this->enableAll || $enableFoo;
    }
}
