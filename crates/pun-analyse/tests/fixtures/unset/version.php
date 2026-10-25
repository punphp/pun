<?php

namespace UnsetVersion;

class Properties
{
    public int $ordinary;
    private int $private;
    /** @final */
    public int $final;
    /** @readonly */
    public int $documented;
    public readonly int $native;

    public function test(): void
    {
        unset($this->ordinary);
        unset($this->private);
        unset($this->final);
        unset($this->documented);
        unset($this->native);
    }
}
