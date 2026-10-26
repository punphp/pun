<?php

class A {
    private bool $myBool;

    public function update(bool $myBool): void {
        $this->myBool = $myBool;
    }
}
