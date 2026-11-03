<?php

declare(strict_types=1);

namespace League\Flysystem;

class UnableToCheckDirectoryExistence extends UnableToCheckExistence
{
    public function operation(): string
    {}
}
