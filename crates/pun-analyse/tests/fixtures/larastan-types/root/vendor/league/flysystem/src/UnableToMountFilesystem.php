<?php

declare(strict_types=1);

namespace League\Flysystem;

use LogicException;

class UnableToMountFilesystem extends LogicException implements FilesystemException
{
    /**
     * @param mixed $key
     */
    public static function becauseTheKeyIsNotValid($key): UnableToMountFilesystem
    {}

    /**
     * @param mixed $filesystem
     */
    public static function becauseTheFilesystemWasNotValid($filesystem): UnableToMountFilesystem
    {}
}
