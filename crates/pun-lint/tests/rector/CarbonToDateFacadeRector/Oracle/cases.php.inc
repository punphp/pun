<?php

namespace App;

use Carbon\Carbon as C;

C::now();
\Illuminate\Support\Carbon::parse('2024-01-01', 'UTC');
Other::now();
-----
<?php

namespace App;

use Carbon\Carbon as C;

\Illuminate\Support\Facades\Date::now();
\Illuminate\Support\Facades\Date::parse('2024-01-01', 'UTC');
Other::now();
