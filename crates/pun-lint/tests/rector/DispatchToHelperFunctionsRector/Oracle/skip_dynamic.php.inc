<?php

namespace App;

use RectorLaravel\Tests\Rector\StaticCall\DispatchToHelperFunctionsRector\Source\TestJob;

$method = 'dispatch';
TestJob::$method('param1');
TestJob::{'dispatch'}('param1');
