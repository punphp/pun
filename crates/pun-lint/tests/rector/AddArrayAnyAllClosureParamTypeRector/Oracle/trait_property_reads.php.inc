<?php

namespace Probe;

trait Files
{
    public $files;

    public function has($files)
    {
        $this->files = $files;
        $files = $this->files;
        $list = [$files];

        return array_any([], fn ($f) => $f !== null) || array_any($list, fn ($f) => $f !== null);
    }
}
