<?php

expect($a)->toBe(1);
expect($a)->toBe(2);-----
<?php

expect($a)->toBe(1)
    ->toBe(2);
