<?php

expect(1.5)->toBeFloat()->toBeScalar()->toBe(1.5);
expect('1')->toBeString()->toBeNumeric()->toBeScalar()->toBe('1');
-----
<?php

expect(1.5)->toBe(1.5);
expect('1')->toBe('1');
