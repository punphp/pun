<?php

stubbed_fn();
new StubbedClass();

class StubSub extends StubReal
{
    public function f(): void
    {
        $this->viaStub();
        $this->notInStub();
    }
}
