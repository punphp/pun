<?php

namespace App;

class Holder {}
interface HolderI {}
abstract class HolderA {}
final class Fin {}

class E1 extends \Exception { public function __construct(string $message = '', int $code = 0, Holder $h = null, ?\Throwable $previous = null) {} }
class E2 extends \Exception { public function __construct(string $message = '', int $code = 0, ?HolderI $h = null, ?\Throwable $previous = null) {} }
class E3 extends \Exception { public function __construct(string $message = '', int $code = 0, ?HolderA $h = null, ?\Throwable $previous = null) {} }
class E4 extends \Exception { public function __construct(string $message = '', int $code = 0, ?\stdClass $h = null, ?\Throwable $previous = null) {} }
class E6 extends \Exception { public function __construct(string $message = '', int $code = 0, ?\Countable $h = null, ?\Throwable $previous = null) {} }
class E7 extends \Exception { public function __construct(string $message, int $code, HolderI $h, ?\Throwable $previous = null) {} }
class E8 extends \Exception { public function __construct(string $message, int $code, HolderI|\Countable|null $h, ?\Throwable $previous = null) {} }
class E9 extends \Exception { public function __construct(string $message, int $code, object $h, ?\Throwable $previous = null) {} }
class E10 extends \Exception { public function __construct(string $message, int $code, \Exception|\Error $h, ?\Throwable $previous = null) {} }
class E12 extends \Exception { public function __construct(string $message, int $code, HolderI|string $h, ?\Throwable $previous = null) {} }
class E14 extends \Exception { public function __construct(string $message, int $code, ?\Exception $h, ?\Throwable $previous = null) {} }
class E15 extends \Exception { public function __construct(string $message, int $code, ?Missing $h, ?\Throwable $previous = null) {} }
class E16 extends \Exception { public function __construct(string $message, int $code, Missing $h, ?\Throwable $previous = null) {} }
class E17 extends \Exception { public function __construct(string $message, int $code, Fin $h, ?\Throwable $previous = null) {} }
class E18 extends \Exception { public function __construct(string $message, int $code, \Countable $h, ?\Throwable $previous = null) {} }
class E19 extends \Exception { public function __construct(string $message, int $code, HolderA $h, ?\Throwable $previous = null) {} }
class E20 extends \Exception { public function __construct($message, $h, $previous = null) {} }

function f($x) {
    try { $x(); } catch (\Throwable $e) { throw new E1('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E2('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E3('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E4('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E6('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E7('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E8('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E9('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E10('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E12('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E14('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E15('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E16('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E17('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E18('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E19('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E20('m', 1); }
    try { $x(); } catch (\Throwable $e) { throw new \ErrorException('m', 1); }
    try { $x(); } catch (\Throwable $e) { throw new \ErrorException('m', previous: $e); }
    try { $x(); } catch (\Throwable $e) { throw new \ErrorException(code: 3, message: 'm'); }
}
-----
<?php

namespace App;

class Holder {}
interface HolderI {}
abstract class HolderA {}
final class Fin {}

class E1 extends \Exception { public function __construct(string $message = '', int $code = 0, Holder $h = null, ?\Throwable $previous = null) {} }
class E2 extends \Exception { public function __construct(string $message = '', int $code = 0, ?HolderI $h = null, ?\Throwable $previous = null) {} }
class E3 extends \Exception { public function __construct(string $message = '', int $code = 0, ?HolderA $h = null, ?\Throwable $previous = null) {} }
class E4 extends \Exception { public function __construct(string $message = '', int $code = 0, ?\stdClass $h = null, ?\Throwable $previous = null) {} }
class E6 extends \Exception { public function __construct(string $message = '', int $code = 0, ?\Countable $h = null, ?\Throwable $previous = null) {} }
class E7 extends \Exception { public function __construct(string $message, int $code, HolderI $h, ?\Throwable $previous = null) {} }
class E8 extends \Exception { public function __construct(string $message, int $code, HolderI|\Countable|null $h, ?\Throwable $previous = null) {} }
class E9 extends \Exception { public function __construct(string $message, int $code, object $h, ?\Throwable $previous = null) {} }
class E10 extends \Exception { public function __construct(string $message, int $code, \Exception|\Error $h, ?\Throwable $previous = null) {} }
class E12 extends \Exception { public function __construct(string $message, int $code, HolderI|string $h, ?\Throwable $previous = null) {} }
class E14 extends \Exception { public function __construct(string $message, int $code, ?\Exception $h, ?\Throwable $previous = null) {} }
class E15 extends \Exception { public function __construct(string $message, int $code, ?Missing $h, ?\Throwable $previous = null) {} }
class E16 extends \Exception { public function __construct(string $message, int $code, Missing $h, ?\Throwable $previous = null) {} }
class E17 extends \Exception { public function __construct(string $message, int $code, Fin $h, ?\Throwable $previous = null) {} }
class E18 extends \Exception { public function __construct(string $message, int $code, \Countable $h, ?\Throwable $previous = null) {} }
class E19 extends \Exception { public function __construct(string $message, int $code, HolderA $h, ?\Throwable $previous = null) {} }
class E20 extends \Exception { public function __construct($message, $h, $previous = null) {} }

function f($x) {
    try { $x(); } catch (\Throwable $e) { throw new E1('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E2('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E3('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E4('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E6('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E7('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E8('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E9('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E10('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E12('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E14('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E15('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E16('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E17('m', 1, null, $e); }
    try { $x(); } catch (\Throwable $e) { throw new E18('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E19('m', 1, null); }
    try { $x(); } catch (\Throwable $e) { throw new E20('m', 1); }
    try { $x(); } catch (\Throwable $e) { throw new \ErrorException('m', 1, $e, $e, $e, $e); }
    try { $x(); } catch (\Throwable $e) { throw new \ErrorException('m', $e->getCode(), previous: $e); }
    try { $x(); } catch (\Throwable $e) { throw new \ErrorException(code: 3, message: 'm', previous: $e); }
}
