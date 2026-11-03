<?php

declare(strict_types=1);

namespace League\Flysystem;

class UnableToCheckFileExistence extends UnableToCheckExistence
{
    public function operation(): string
    {}
}
