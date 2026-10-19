<?php

round(1.5, 0, \PHP_ROUND_HALF_UP);
round(1.5, precision: 0, mode: PHP_ROUND_HALF_EVEN);
round(1.5, 0, php_round_half_up);
round(...$a);
round(1.5, 0, PHP_ROUND_HALF_ODD, 1);
\round(2.5, 1, PHP_ROUND_HALF_DOWN);
-----
<?php

round(1.5, 0, \RoundingMode::HalfAwayFromZero);
round(1.5, precision: 0, mode: \RoundingMode::HalfEven);
round(1.5, 0, php_round_half_up);
round(...$a);
round(1.5, 0, PHP_ROUND_HALF_ODD, 1);
\round(2.5, 1, \RoundingMode::HalfTowardsZero);
