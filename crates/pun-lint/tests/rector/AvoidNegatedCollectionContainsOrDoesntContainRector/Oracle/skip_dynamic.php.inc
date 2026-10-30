<?php

namespace App;

use Illuminate\Support\Collection;

$collection = new Collection([0, 1, null, -1]);
$method = 'contains';
! $collection->$method(1);
! $collection->{'contains'}(1);
