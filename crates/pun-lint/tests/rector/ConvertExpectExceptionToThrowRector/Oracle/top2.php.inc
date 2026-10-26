<?php

$x  =  1;

$this->expectException(RuntimeException::class);
doSomething();
-----
<?php

$x  =  1;
expect(fn() => doSomething())->toThrow(RuntimeException::class);
