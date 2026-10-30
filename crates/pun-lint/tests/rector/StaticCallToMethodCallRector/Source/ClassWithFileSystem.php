<?php

declare(strict_types=1);

namespace Rector\Tests\Transform\Rector\StaticCall\StaticCallToMethodCallRector\Source;

abstract class ClassWithFileSystem
{
    /**
     * @var TargetFileSystem
     */
    public $smartFileSystemProperty;
}
