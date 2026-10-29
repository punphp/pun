<?php

namespace App\Q;

use CompanyB\Queue as Line;

Line::run();
\Queue::push();
-----
<?php

namespace App\Q;

use CompanyB\Queue as Line;

Line::run();
\Illuminate\Support\Facades\Queue::push();
