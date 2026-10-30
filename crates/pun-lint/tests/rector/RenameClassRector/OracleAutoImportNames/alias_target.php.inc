<?php

namespace App\Alias;

use Illuminate\Support\Facades\Storage as Disk;

Disk::get();
\Storage::put();
-----
<?php

namespace App\Alias;

use Illuminate\Support\Facades\Storage as Disk;

Disk::get();
Disk::put();
