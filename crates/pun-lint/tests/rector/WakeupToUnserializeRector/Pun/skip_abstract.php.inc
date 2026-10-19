<?php

// Rector gives the abstract method a body, which PHP rejects; Pun leaves it alone.
abstract class A
{
    abstract public function __wakeup();
}
