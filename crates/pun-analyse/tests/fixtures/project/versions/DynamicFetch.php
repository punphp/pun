<?php

namespace Versions;

class DynamicFetch
{
    public const FOO = 1;

    public function fetch(): void
    {
        echo self::{$this};
        echo $this::{$this};
        $f = static function () {
            echo DynamicFetch::{$this};
        };
    }
}
