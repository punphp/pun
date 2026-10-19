<?php

/** @internal */
class GlobalSecret
{
    public function run(): void
    {
    }

    public function self(): void
    {
        $this->run();
    }
}

class GlobalCaller
{
    public function call(GlobalSecret $secret): void
    {
        $secret->run();
    }
}
