<?php

throw_if($a, new Foo\Bar);
throw_unless($a, new static(1, two: 2));
throw_if($a, new $class('m'));
throw_if($a, Foo::make());
THROW_IF($a, new \Exception(...$args));
throw_if($a, new class {});
throw_if($a, exception: new E('m'));
throw_if($a, new E('m'), 'x');
-----
<?php

throw_if($a, Foo\Bar::class);
throw_unless($a, static::class, 1, two: 2);
throw_if($a, new $class('m'));
throw_if($a, Foo::make());
THROW_IF($a, \Exception::class, ...$args);
throw_if($a, new class {});
throw_if($a, E::class, 'm');
throw_if($a, new E('m'), 'x');
