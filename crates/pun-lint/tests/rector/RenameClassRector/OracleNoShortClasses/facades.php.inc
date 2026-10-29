<?php

namespace App;

use Storage;

Storage::disk();
\Queue::push();
-----
<?php

namespace App;

use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Queue;

Storage::disk();
Queue::push();
