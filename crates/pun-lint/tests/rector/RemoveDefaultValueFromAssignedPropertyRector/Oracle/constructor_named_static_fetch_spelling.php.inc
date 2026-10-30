<?php

namespace ConstructorFetchSpelling;

final class ExactFetch
{
    private int $value = 0;

    public function __construct()
    {
        ExactFetch::$value = 1;
    }
}

final class MismatchedFetch
{
    private int $value = 0;

    public function __construct()
    {
        mIsMaTcHeDfEtCh::$value = 2;
    }
}

final class RelativeFetch
{
    private int $value = 0;

    public function __construct()
    {
        namespace\RelativeFetch::$value = 3;
    }
}
-----
<?php

namespace ConstructorFetchSpelling;

final class ExactFetch
{
    private int $value;

    public function __construct()
    {
        ExactFetch::$value = 1;
    }
}

final class MismatchedFetch
{
    private int $value = 0;

    public function __construct()
    {
        mIsMaTcHeDfEtCh::$value = 2;
    }
}

final class RelativeFetch
{
    private int $value;

    public function __construct()
    {
        namespace\RelativeFetch::$value = 3;
    }
}
