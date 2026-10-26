<?php

uses(Tests\TestCase::class, X::FOO, X::{'class'}, $x::class, static::class, Tests\Other::class, Tests\Concerns\T::class);
-----
<?php

uses(X::FOO, X::{'class'}, $x::class, static::class, Tests\Other::class, Tests\Concerns\T::class);
