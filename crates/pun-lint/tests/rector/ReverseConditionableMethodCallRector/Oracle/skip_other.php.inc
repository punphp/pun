<?php

namespace App;

use RectorLaravel\Tests\Rector\MethodCall\ReverseConditionableMethodCallRector\Source\ConditionableExample;

$conditionable = new ConditionableExample();
$method = 'when';

$conditionable->$method(!true, function () {});
$conditionable->other(!true, function () {});
$conditionable->when(-1, function () {});
$conditionable->unless(~1, function () {});
