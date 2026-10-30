<?php

namespace App;

use RectorLaravel\Tests\Rector\FuncCall\RemoveRedundantValueCallsRector\Source\HelperObject;

strval(new HelperObject());
other(new HelperObject())->store();
