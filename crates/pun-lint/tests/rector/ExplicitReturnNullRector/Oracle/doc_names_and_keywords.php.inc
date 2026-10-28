<?php

namespace Oracle\ExplicitNull2;

final class Runner
{
    /** @return Baz|void */
    public function a(int $n) { if ($n) { return 1; } }

    /** @return self|void */
    public function b(int $n) { if ($n) { return 1; } }

    /** @return boolean|void */
    public function c(int $n) { if ($n) { return 1; } }

    /** @return true|false|void */
    public function d(int $n) { if ($n) { return 1; } }

    /** @return null|void */
    public function e(int $n) { if ($n) { return 1; } }

    /**
     * @return string|void
     * @phpstan-return non-empty-string|void
     */
    public function f(int $n) { if ($n) { return 'x'; } }

    /** @return String|VOID */
    public function g(int $n) { if ($n) { return 'x'; } }
}
-----
<?php

namespace Oracle\ExplicitNull2;

final class Runner
{
    /** @return Oracle\ExplicitNull2\Baz|null */
    public function a(int $n) { if ($n) { return 1; }
    return null; }

    /** @return Oracle\ExplicitNull2\Runner|null */
    public function b(int $n) { if ($n) { return 1; }
    return null; }

    /** @return bool|null */
    public function c(int $n) { if ($n) { return 1; }
    return null; }

    /** @return bool|null */
    public function d(int $n) { if ($n) { return 1; }
    return null; }

    /** @return null|void */
    public function e(int $n) { if ($n) { return 1; }
    return null; }

    /**
     * @return string|null
     * @phpstan-return non-empty-string|void
     */
    public function f(int $n) { if ($n) { return 'x'; }
    return null; }

    /** @return string|null */
    public function g(int $n) { if ($n) { return 'x'; }
    return null; }
}
